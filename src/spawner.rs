pub struct Spawner<T: Bundle + Default> {
    pub bundle: T,
    pub timer: Timer,
}
