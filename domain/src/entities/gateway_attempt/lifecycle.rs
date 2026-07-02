

pub trait AttemptLifecycle{
    fn is_running(&self) -> bool;
    fn is_success(&self) -> bool;
    fn is_failure(&self) -> bool;
}