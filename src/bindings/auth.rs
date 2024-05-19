use crate::app::FirebaseApp;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Auth;

    // this is for compat
    #[wasm_bindgen(method, js_name = auth)]
    pub fn auth(_: &FirebaseApp) -> Auth;

    // This gets the current user
    #[derive(Debug)]
    pub type User;
    #[wasm_bindgen(method, getter, js_name = currentUser)]
    pub fn current_user(this: &Auth) -> User;

    // this is for the new version of
    #[wasm_bindgen(method, js_name = onAuthStateChanged)]
    pub fn on_auth_state_changed(this: &Auth, callback: &Closure<dyn FnMut(JsValue)>);

    #[wasm_bindgen(method, catch, js_name = createUserWithEmailAndPassword)]
    pub async fn create_user_with_email_and_password(
        this: &Auth,
        email: String,
        password: String,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = signInWithEmailAndPassword)]
    pub async fn sign_in_with_email_and_password(
        this: &Auth,
        email: String,
        password: String,
    ) -> Result<JsValue, JsValue>;

    // There is an optional actionCodeSettings parameter that is not implemented here
    #[wasm_bindgen(catch, method, js_name = sendPasswordResetEmail)]
    pub async fn send_password_reset_email(this: &Auth, email: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = signOut)]
    pub async fn sign_out(this: &Auth) -> Result<JsValue, JsValue>;

    // Google Auth support
    pub type GoogleAuthProvider;

    #[wasm_bindgen(constructor, js_namespace = ["firebase", "auth"])]
    pub fn new() -> GoogleAuthProvider;

    // Github Auth support
    // the name is incorrect in firebase so we have to use
    // Github here and create another tpye alias for it
    pub type GithubAuthProvider;

    #[wasm_bindgen(constructor, js_namespace = ["firebase", "auth"])]
    pub fn new() -> GithubAuthProvider;

    #[wasm_bindgen(catch, method, js_name = signInWithPopup)]
    pub async fn sign_in_with_popup_google(
        auth: &Auth,
        provider: &GoogleAuthProvider,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = signInWithPopup)]
    pub async fn sign_in_with_popup_github(
        auth: &Auth,
        provider: &GitHubAuthProvider,
    ) -> Result<JsValue, JsValue>;
}

// type alias
pub type GitHubAuthProvider = GithubAuthProvider;
