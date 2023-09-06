

use proc_macro::TokenStream;
use quote::quote;



#[proc_macro_derive(ActionMacro)]
pub fn action_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_action_macro(&ast)
}

fn impl_action_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[async_trait]
        impl Action for #name {
            fn name(&self) -> &str{
                stringify!(#name)
            }
            fn set_prefix(&mut self, prefix: &str, profile: &str) {
                self.prefix = prefix.into();
                self.profile = profile.into();
            }
            fn get_prefix(&self) -> &str{
                &self.prefix
            }
            async fn aask(&self, prompt: &str) -> String{
                self._llm.aask(prompt.into()).await
            }
            /// 这里接收的是 所有信息，但是不是所有行为都会用到
            /// 有些只需要一条，所以使用条件有限制
            /// TODO 需要重新设计这里
            async fn run(&self, msgs: Vec<&Message>)-> String {

                let prompt = self._build_prompt(msgs.clone()).await;
                debug!("{:?}", self);
                info!("【{} Prompt】: \n {}", stringify!(#name), &prompt);
                // 测试数据
                if std::env::var("LLM_FAKE").is_ok() && std::env::var("LLM_FAKE").unwrap() == "true"  {
                    // return PROMPT_TEMPLATE_RESPONSE_SAMPLE_FULL.into();
                    return self._post_processing(msgs, PROMPT_TEMPLATE_RESPONSE_SAMPLE_FULL.into()).await;
                }
                let llm_response = self.aask(&prompt).await;
                self._post_processing(msgs, llm_response).await
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(RoleMacro)]
pub fn role_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_role_macro(&ast)
}

fn impl_role_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {

        #[async_trait]
        impl Role for #name {

            fn set_env_global_memory(&mut self, memory: Arc<Mutex<Memory>>) {
                self._rc.env_memory = memory
            }
            
            fn _reset(&mut self) {
                self._states = vec![];
                self._actions = vec![];
            }

            fn _init_actions(&mut self, actions: Vec<Box<dyn Action>>) {
                self._reset();
                for mut action in actions {
                    action.set_prefix(&self._setting.get_prefix(), &self._setting.profile);
                    self._actions.push(action);
                }
            }

            fn _watch(&mut self, actions: Vec<Box<dyn Action>>) {

            }
            fn _set_state(&mut self, state: i32) {
                // let mut _rc = self._get_role_context();
                // _rc.state = state;
            }
            fn _get_profile(&self) -> &str {
                &self._setting.profile
            }
            /// """获取角色前缀""" 
            fn _get_prefix(&self) -> String {
            self._setting.get_prefix()
            }

            fn _get_states(&self) ->Vec<String> {
                self._states.clone()
            }
            fn _get_rc(&self) -> RoleContext {
                self._rc.clone()
            }
            fn _get_rc_env_memory(&self) -> MutexGuard<'_, Memory> {
                // 获取可变引用并锁定 Mutex
                debug!("_get_rc_env_memory, have {:?} messages", self._rc.env_memory.lock().unwrap().count());
                self._rc.env_memory.lock().unwrap()
            }
            fn _get_rc_memory(&self) -> MutexGuard<'_, Memory> {
                self._rc.role_memory.lock().unwrap()
            }
            fn _get_action_by_state(&self, state: usize) -> Option<&Box<dyn Action>> {
                let Some(action) = self._actions.get(state) else { return None };
                Some(action)
            }

            fn _get_action_count(&self) -> usize {
                self._actions.len()
            }

            fn _before_action(&self, env_msgs: &Vec<Message>,  role_msgs: &Vec<Message>) -> String {
                self._before_action(env_msgs, role_msgs);
                String::new()
            }

            fn _after_action(&self, message: Message) -> Message {
                self._after_action(message)
            }
        }
    };
    gen.into()
}