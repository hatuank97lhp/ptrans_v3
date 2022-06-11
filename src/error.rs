use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError{val: String},

    #[error("DenominationInvalid Invalid")]
    DenominationInvalid {},

    #[error("Commitment submitted")]
    CommitmentSubmitted {},

    #[error("Please send `mixDenomination` ORAI along with transaction")]
    InvalidDenom{},
    
    #[error("Fee exceeds transfer value")]
    FeeExceed{},

    #[error("The note has been already spent")]
    InvalidRoot{},

    #[error("Cannot find your merkle root")]
    NullifierUsed{},

    #[error("Invalid withdraw proof")]
    InvalidProof{},

    #[error("Message value is supposed to be zero for ORAI instance")]
    InvalidValue{},

    #[error("Refund value is supposed to be zero for ORAI instance")]
    InvalidRefund{},

    #[error("payment to recipient did not go thru")]
    FailedToRecipient{},

    #[error("payment to _relayer did not go thru")]
    FailedToRelayed{},

    #[error("Levels Invalid")]
    LevelsInvalid {},

    #[error("Merkle Tree Full")]
    MerkleTreeFull {},
}
