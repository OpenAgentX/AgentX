pub trait Relationship: Sized + Send + Sync + 'static{
}


/// 感知组件可以从环境中获取消息
pub trait PerceptionComponent: Relationship {

}
