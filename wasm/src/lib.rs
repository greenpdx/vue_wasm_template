use serde_json::Value;

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct Config {
    #[wasm_bindgen(skip)]
    pub fetch_url: String,
    #[wasm_bindgen(skip)]
    pub passwd: String,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    unsafe fn prompt(s: &str) -> String;
}

#[wasm_bindgen]
pub fn greet() -> String {
    let mut passwd: String;
    unsafe {
        passwd = prompt("Enter Password!");
    };
    passwd
}

#[wasm_bindgen]
pub fn init(env: JsValue) -> Result<Config, JsValue> {
    //console::log_1(&env);
    /*let passwd = match web_sys::window() {
        None => return Err(JsValue::from_str("No Window")),
        Some(w) => {
            w.prompt_with_message("Enter Password").unwrap().unwrap()
        }
    };*/
    let passwd = "test".to_string(); //greet();

    let e: Value = serde_wasm_bindgen::from_value(env).unwrap();
    let o = e.as_object().unwrap();
    
    let fetch_url = o.get("VITE_FETCH_URL").unwrap().as_str().unwrap();
    
    let conf = Config {
        fetch_url: fetch_url.to_string(),
        passwd: passwd,
    };
    Ok(conf)
}

#[wasm_bindgen]
pub fn tstpass(conf: &Config) -> Result<JsValue, JsValue> {
    let passwd = &conf.passwd;
    Ok(JsValue::from_str(&passwd))
}

use wasm_bindgen_futures::JsFuture;
//use wasm_bindgen_test::console_log;
use web_sys::{ console, window, Request, RequestInit, RequestMode, Response};


pub async fn run_rest(conf: &Config, method: &str, path: String, jsopts: JsValue, body: JsValue, ) -> Result<JsValue, JsValue> {
    // jsopts: JsValue to Vec<(String, String)>
    let fetch = conf.fetch_url.clone();

    let mut url = fetch + &path;

    if jsopts.is_array() {
        match serde_wasm_bindgen::from_value::<Vec<(String, String)>>(jsopts) {
            Ok(os) => {
                url = url + "?" ;
                for (k,v) in os {
                    url = url + &k + "=" + &v + "&";
                };
                let len = url.len();
                url = url[0..(len-1)].to_string();
                console::log_1(&JsValue::from_str(&url));
            },
            Err(e) => return Err(JsValue::from_str("Bad Opts"))
        };
    };
    

    let ropts = RequestInit::new();
    ropts.set_method(&method);
    ropts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &ropts)?;

    let headers = request.headers();
    headers.set("Content-Type", "application/json")?;
    headers.set("Accept", "application/json")?;
    headers.set("Access-Control-Allow-Origin", "*")?;

    let window = window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let data = resp.json()?; 
    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(data).await?;
    Ok(json)
    //Ok(JsValue::from_str("s"))
}
