#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gccd");

    println!("Serving on http://localhost:3000..");
    //listen on TCP port 3000. Get_form is the callback function that is handling all the request.
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
              <input type="text" name="n"/>
              <input type="text" name="n"/>
              <button type="submit">Compute GCD</button>
</form> "#,
    );

    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    // request.get_ref::<UrlEncodedBody> parse the body as a table mapping query parameter names to
    // arrays of values.
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:#?}", e));
            return Ok(response);
        }
        Ok(map) => map,
    };

    //finds n within this table which is where the HTML form places the numbers entered into the
    //webpage.
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };

    //parse the numbers into the vector.
    let mut numbers: Vec<u64> = Vec::new();
    for num in unparsed_numbers {
        match num.parse() {
            Ok(n) => {
                numbers.push(n);
            }
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!(
                    "Value for 'n' parameter not a number: {:#?}\n",
                    num
                ));
                return Ok(response);
            }
        }
    }

    //call our gcd function.
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    //set the response and return the Ok variant.
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset= Utf8));
    response.set_mut(format!(
        "The greatest comon divisor of the numer {:?} is <b>{}</b>\n",
        numbers, d,
    ));

    Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
