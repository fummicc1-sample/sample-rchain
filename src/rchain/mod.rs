extern crate blake2;

#[derive(Debug, Clone)]
pub struct Blockchain {
    /// Store all accepted blocks
    pub blocks: Vec<Block>,

    /// Lookup from AccountID (Public Key) to Account
    pub accounts: HashMap<String, Account>,

    /// internal
    /// transactions which should be added to block, but have not.
    pendingTransactions: Vec<Transaction>,
}

trait WorldState {
    /// All accountId stored in Blockchain
    fn getUserIds(&self) -> Vec<String>;

    /// Return a specific mutable account with ID    
    fn getMutableAccountById(&mut self, id: &String) -> Option<&mut Account>;

    /// Return a specific account with ID
    fn getAccountById(&self, id: &String) -> Option<&Account>;

    /// Add a new account
    fn createAccount(&mut self, id: String, account_type: AccountType) -> Result<(), &'static str>;
}

/// A single part of Blockchain
/// Storeing a list of transactions
#[derive(Debug, Clone)]
pub struct Block {
    /// Length must be greater than one
    pub(crate) transactions: Vec<Transaction>,
    previousBlockHash: String,
    hash: String,
    /// Used for proof of work
    nonce: u128,
}

/// Account on the blockchain
#[derive(Clone, Debug)]
pub struct Account {
    /// Multiple account information,
    /// in case of storing different kind of digital values
    /// (e.g., purchases of digital music)
    store: HashMap<String, String>,

    /// To check whether this is account or sth.
    accountType: AccountType,

    /// Total amount of account's tokens
    tokens: u128,
}

#[derive(Clone, Debug)]
pub enum AccountType {
    User,
    Contract,
    Validator {
        correctlyValidatedBlocks: u128,
        incorrectlyValidatedBlocks: u128,
        youGetTheIdea: bool,
    },
}

/// A request to the Blockchain
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Unique number used for randomization
    nonce: u128,

    /// AccountId
    from: String,

    /// The time the transaction was created
    createdAt: SystemTime,

    /// The type of the transaction and its additional infromation
    pub(crate) record: TransactionData,

    /// Signature of the hash of the whole message
    signature: Option<String>,
}

/// A family of single operation on the chain
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionData {
    /// Store a new user account
    CreateUserAccount(String),

    /// Change stored Account information
    ChangeStoreValue { key: String, value: String },

    /// Move tokens from one owner to another
    TransferToken { to: String, amount: u128 },

    /// Give tokens to receiver
    CreateTokens { receiver: String, amount: u128 },
}
