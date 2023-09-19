pub trait Component: Sized + Send + Sync + 'static{
}


/// 感知组件可以从环境中获取消息
pub trait PerceptionComponent: Component {

}


/// 规划组件主要从环境和记忆中规划下一步的行为
/// - Thinking
/// - Reflection
/// - Self-critics
/// - Chain of thoughts
/// - Subgoal decomposition
pub trait PlanningComponent: Component {
// Reflection
// Self-critics
// Chain of thoughts
// Subgoal decomposition
}


/// 行为组件可以使用的行为
pub trait ActionComponent: Component {

}


/// 记忆组件可以存储记忆
/// - retrieve
/// - Short-term memory
/// - Long-term memory
pub trait MemoryComponent: Component {
// retrieve
// Short-term memory
// Long-term memory

}

/// 工具组件可以使用工具集
pub trait ToolComponent: Component {

}

/// 归纳总结组件
pub trait SummarizationComponent: Component {

}


struct Velocity;

impl Component for Velocity {

}

impl PlanningComponent for Velocity {

}