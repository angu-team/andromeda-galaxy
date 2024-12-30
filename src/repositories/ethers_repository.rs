use async_broadcast::{broadcast, Receiver};
use ethers::middleware::Middleware;
use ethers::providers::{Provider, Ws};
use ethers::types::{Block, H256};
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::spawn;

pub struct EthersRepository {
    connections: HashMap<i32, Arc<Provider<Ws>>>,
    block_listeners: HashMap<i32, Receiver<Block<H256>>>,
}

impl EthersRepository {
    pub fn new() -> Self {
        EthersRepository {
            connections: HashMap::new(),
            block_listeners: HashMap::new(),
        }
    }

    /// Retorna o recebedor (Receiver) de blocos correspondente ao usuário especificado pelo `user_id`.
    ///
    /// Para ouvir os eventos disparados, você pode usar o `Receiver` retornado por esta função para
    /// consumir os blocos conforme forem sendo enviados.
    ///
    /// Exemplo:
    ///
    /// ```rust
    /// if let Some(mut receiver) = get_block_listener(user_id) {
    ///     tokio::spawn(async move {
    ///         while let Ok(block) = receiver.recv().await {
    ///             println!("Recebi um bloco: {:?}", block);
    ///         }
    ///     });
    /// } else {
    ///     println!("Nenhum listener configurado para o user_id: {}", user_id);
    /// }
    /// ```
    ///
    /// - A função retorna `Some(receiver)` se o `user_id` estiver registrado com um listener.
    /// - Caso contrário, retorna `None`, indicando que o `user_id` não possui um listener configurado.
    pub fn get_block_listener(&self, user_id: i32) -> Option<Receiver<Block<H256>>> {
        self.block_listeners.get(&user_id).cloned()
    }

    /// Essa função registra e armazena um `Provider<Ws>` (conexão WebSocket) associado a um usuário específico.
    ///
    /// **Passo a passo**:
    /// 1. A conexão `provider` é encapsulada dentro de um `Arc`, que é um ponteiro inteligente
    ///    usado para permitir compartilhamento seguro de dados entre threads.
    /// 2. A conexão encapsulada (`Arc<Provider<Ws>>`) é inserida no mapa `connections`,
    ///    associando-a ao identificador do usuário `user_id`.
    ///
    /// Esse método é útil para gerenciar múltiplas conexões de usuários, permitindo que cada um
    /// tenha sua própria instância de `Provider<Ws>` armazenada e pronta para uso.
    pub fn apply_connection(&mut self, user_id: i32, provider: Provider<Ws>) {
        let provider = Arc::new(provider);
        self.connections.insert(user_id, provider);
    }

    /// Essa função aplica um "listener" de blocos para o usuário especificado, utilizando um `Provider<Ws>` como fonte.
    ///
    /// - Um canal de broadcast é criado com capacidade de 10 mensagens, onde múltiplos receptores podem subscrever para receber blocos.
    /// - Um clone do canal de transmissão (`tx`) é criado para ser usado dentro do escopo da tarefa assíncrona.
    ///
    /// **Passo a passo do funcionamento**:
    /// 1. Um clone do `Provider<Ws>` (`block_subscriber`) é usado para assinar os blocos em tempo real.
    /// 2. Uma nova tarefa assíncrona é criada com `spawn`. Dentro dela:
    ///     - O stream de blocos é iniciado usando o método `subscribe_blocks()` do provider.
    ///     - O método `for_each` percorre os blocos recebidos.
    ///     - Para cada bloco:
    ///         * Um clone do `tx` é criado (`tx_clone_inner`) para garantir que a transmissão dentro do escopo seja possível.
    ///         * O bloco é transmitido para todos que estejam "ouvindo" o canal de broadcast.
    /// 3. No final, o receptor (`rx1`) do canal de broadcast é armazenado no mapa `block_listeners` para o usuário correspondente.
    ///    Isso permite que múltiplos consumidores recebam os blocos em paralelo.
    pub async fn apply_block_listener(&mut self, user_id: i32, provider: Provider<Ws>) {
        let block_subscriber = provider.clone();

        let (mut tx, rx1) = broadcast(10); // canal com capacidade 10
        let tx_clone = tx.clone(); // Faz o primeiro clone

        spawn(async move {
            let stream = block_subscriber.subscribe_blocks().await.unwrap();

            stream
                .for_each(move |block| {
                    let tx_clone_inner = tx_clone.clone(); // Criamos o clone de tx_clone dentro do escopo do for_each
                    async move {
                        tx_clone_inner.broadcast(block).await.unwrap();
                    }
                })
                .await;
        });

        self.block_listeners.insert(user_id, rx1);
    }

    /// Retorna a conexão WebSocket (`Arc<Provider<Ws>>`) associada ao `user_id` fornecido.
    ///
    /// Essa conexão pode ser utilizada para interagir com um nó Ethereum via WebSocket, permitindo
    /// realizar chamadas ou escutar eventos relacionados a esse nó.
    ///
    /// Exemplo:
    ///
    /// ```rust
    /// if let Some(provider) = get_connection(user_id) {
    ///     // Usa a conexão para fazer chamadas, por exemplo:
    ///     let balance = provider.get_balance("endereco_ethereum", None).await.unwrap();
    ///     println!("Saldo: {}", balance);
    /// } else {
    ///     println!("Nenhuma conexão configurada para o user_id: {}", user_id);
    /// }
    /// ```
    ///
    /// - A função retorna `Some(Arc<Provider<Ws>>)` se o `user_id` estiver associado a uma conexão existente.
    /// - Caso contrário, retorna `None`, indicando que o `user_id` não possui uma conexão registrada.
    ///
    /// **Nota:** A conexão é encapsulada em um `Arc` (`Atomic Reference Counter`), permitindo que ela seja
    /// compartilhada de forma segura entre várias threads ou tasks assíncronas sem a necessidade de clones caros.
    pub fn get_connection(&self, user_id: i32) -> Option<Arc<Provider<Ws>>> {
        self.connections.get(&user_id).cloned()
    }
}
