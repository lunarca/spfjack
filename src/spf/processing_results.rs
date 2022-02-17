use decon_spf::spf::mechanism::Mechanism;

pub struct MechanismProcessingResult<T> {
    mechanism_type: MechanismType,
    issue: MisconfigType,
    mechanism: Mechanism<T>
}

pub enum MechanismType {
    All,
    Ip4,
    Ip6,
    A,
    Mx,
    Ptr,
    Exists,
    Include,
    Redirect,
}

pub enum MisconfigType {
    /// Mechanism is +all
    PlusAll,
    /// Mechanism points to an open relay
    OpenRelay(MechanismType, String), //Consider union type of DNS name or IP4/6 address
    /// Mechanism points to an unregistered domain
    UnregisteredDomain(MechanismType, String) 
}