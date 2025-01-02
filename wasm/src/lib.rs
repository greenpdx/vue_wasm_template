use serde_json::Value;
use serde::{Serialize, Deserialize};
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[wasm_bindgen(skip)]
    pub fetch_url: String,
    #[wasm_bindgen(skip)]
    pub passwd: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WorkerRequest {
    pub id: String,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WorkerResponse {
    pub id: String,
    pub rply: String,
}

#[wasm_bindgen]
pub fn wtest(data: JsValue) -> Result<JsValue, JsValue> {
    let data: WorkerRequest = serde_wasm_bindgen::from_value(data)?;
    let resp = WorkerResponse { id: data.id, rply: data.msg };
    Ok(serde_wasm_bindgen::to_value(&resp)?)
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
pub fn tsturl(conf: &Config) -> Result<JsValue, JsValue> {
    let fetch = &conf.fetch_url;
    Ok(JsValue::from_str(&fetch))
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

use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{Element, Document, HtmlElement, HtmlInputElement,MessageEvent, Worker};

#[wasm_bindgen]
pub struct NumberEval {
    number: i32,
}

#[wasm_bindgen]
impl NumberEval {
    /// Create new instance.
    pub fn new() -> NumberEval {
        NumberEval { number: 0 }
    }

    /// Check if a number is even and store it as last processed number.
    ///
    /// # Arguments
    ///
    /// * `number` - The number to be checked for being even/odd.
    pub fn is_even(&mut self, number: i32) -> bool {
        self.number = number;
        self.number % 2 == 0
    }

    /// Get last number that was checked - this method is added to work with
    /// statefulness.
    pub fn get_last_number(&self) -> i32 {
        self.number
    }
}

#[wasm_bindgen]
pub fn startup() -> Result<(), JsValue> {
  
    // Here, we create our worker. In a larger app, multiple callbacks should be
    // able to interact with the code in the worker. Therefore, we wrap it in
    // `Rc<RefCell>` following the interior mutability pattern. Here, it would
    // not be needed but we include the wrapping anyway as example.
    let worker = Worker::new("/worker.ts")?; // {
    //  Ok(w) => { w },
    //  Err(e) => {
     // }
    //};
    let worker_handle = Rc::new(RefCell::new(worker));
    console::log_1(&"Created a new worker from within Wasm".into());

    // Pass the worker to the function which sets up the `oninput` callback.
    setup_input_oninput_callback(worker_handle);
    Ok(())
}

fn setup_input_oninput_callback(worker: Rc<RefCell<web_sys::Worker>>) {
    let document = window().unwrap().document().unwrap();
 
  // If our `onmessage` callback should stay valid after exiting from the
  // `oninput` closure scope, we need to either forget it (so it is not
  // destroyed) or store it somewhere. To avoid leaking memory every time we
  // want to receive a response from the worker, we move a handle into the
  // `oninput` closure to which we will always attach the last `onmessage`
  // callback. The initial value will not be used and we silence the warning.
  #[allow(unused_assignments)]
  let mut persistent_callback_handle = get_on_msg_callback();

  let callback = Closure::new(move || {
      console::log_1(&"oninput callback triggered".into());
      let document = web_sys::window().unwrap().document().unwrap();

      let input_field = document
          .get_element_by_id("inputNumber")
          .expect("#inputNumber should exist");
      let input_field = input_field
          .dyn_ref::<HtmlInputElement>()
          .expect("#inputNumber should be a HtmlInputElement");

      // If the value in the field can be parsed to a `i32`, send it to the
      // worker. Otherwise clear the result field.
      match input_field.value().parse::<i32>() {
          Ok(number) => {
              // Access worker behind shared handle, following the interior
              // mutability pattern.
              let worker_handle = &*worker.borrow();
              let _ = worker_handle.post_message(&number.into());
              persistent_callback_handle = get_on_msg_callback();

              // Since the worker returns the message asynchronously, we
              // attach a callback to be triggered when the worker returns.
              worker_handle
                  .set_onmessage(Some(persistent_callback_handle.as_ref().unchecked_ref()));
          }
          Err(_) => {
              let ele = document
                  .get_element_by_id("resultField")
                  .expect("#resultField should exist");
                console::log_1(&"the".into());
              ele.dyn_ref::<HtmlElement>()
                  .expect("#resultField should be a HtmlInputElement")
                  .set_inner_text("");
          }
      }
  });
  console::log_1(&"here".into());
  
  // Attach the closure as `oninput` callback to the input field.
  let ele = document
      .get_element_by_id("inputNumber")
      .expect("#inputNumber should exist");
    console::log_1(&"the".into());

    ele.dyn_ref::<HtmlInputElement>()
      .expect("#inputNumber should be a HtmlInputElement")
      .set_oninput(Some(callback.as_ref().unchecked_ref()));
    
  console::log_1(&"next".into());

  // Leaks memory.
  callback.forget();
}

/// Create a closure to act on the message returned by the worker
fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
  console::log_1(&"cb".into());

  Closure::new(move |event: MessageEvent| {
      console::log_2(&"Received response: ".into(), &event.data());

      let result = match event.data().as_bool().unwrap() {
          true => "even",
          false => "odd",
      };

      let document = web_sys::window().unwrap().document().unwrap();
      document
          .get_element_by_id("resultField")
          .expect("#resultField should exist")
          .dyn_ref::<HtmlElement>()
          .expect("#resultField should be a HtmlInputElement")
          .set_inner_text(result);
  })
}