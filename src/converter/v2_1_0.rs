//! converts a postman v 2.1.0 collection to RestClient http format

use crate::{error::PostresError, postman::v2_1_0, restclient};
use fancy_regex::{Captures, Regex};

use super::make_base_name;

mod body;
mod header;
mod method;
mod request;
mod url;

use request::convert_request;

pub(crate) fn convert_to_http(spec: &v2_1_0::PostmanCollection) -> restclient::HttpRequests {
    // postman spec is confusing, ence comments to help you to figure out what is happening
    // Root object has a list of items whose name is item. Items can represent either requests or folders of requests
    // As http files have no concept of folders, we will convert the folder structure to a plain list of requests and try to diferentiate which request is in which folder through request name in http file

    /*
        def 010: iterators
        Let's talk about iterators
        This is going to be a long read, but I really hope it will be worthy.
        In a very summarized way, iterators let you iterate (go through) all items of a collection
        Collections might be lists, maps, anything that holds whatever thing
        In rust, collections usually offer some ways of obtaining an iterator that walks through its items.
        These ways are often provided by methods with the standard name of iter() and into_iter().
        Iterators are, again, kind of simple: you call the next() method on them and this method either returns Some(item) with the next item or None, in which case there are no more items.
        In fact, for loops are just a sugar syntax to obtaining an iterator on the wanted collection and calling next on it until None is returned.
        The difference between iter() and into_iter() is that in the first case items are returned as references, so that you can usually read items, but not change them.
        In the other hand, into_iter() consumes the collection, which means that you are free to use its items as you wish. However, after into_iter obtains an iterator to the collection, nobody can use the collection anymore
        Iterators offer several transformers. These functions usually make some transformations and return another interator, which can go yet through another transformer and so on.
        This allows for a very specific kind of processing, in which a given item goes through several transformers and, at the end, is collected by a collector.
        Collectors run an iterator and process their items, generating a result.
        What this result is will deppend on the collector used.
        Very abstract?
        Well, let's go through an example implementation: suppose we have the following collection:
        [1, 2, 3, 4, 5, 6, 7]
        Our goal is to iterate through all these elements and double them.
        After doubling them, we want to eliminate all items greater than 6.
        Finally, we want to convert the filtered elements to characters and put them in a vector of characters
        let vec_of_chars: Vec<String> = vec![1, 2, 3, 4, 5, 6, 7]
        // we have the vec of usizes
        .iter()
        // now, we have an iterator
        .map( | i | i * 2)
        // .map acts on an iterator calling an item from the original iterator and applying the inline closure to it. Map returns an iterator with the next item set to the transformed item
        .filter( | i | I <= 6)
        // .filter acts on an iterator. It consumes items from the original iterator in sequence and applies a test on them. The test is defined in the closure, it should return either true or false.
        // if the test rreturns true, filter returns an iterator with the next item equals to the item which passed the test. If the test returns false, the next item of the original iterator is tested and this process repeats until either an item from the original iterator passes the test or the original iterator returns None, in which case filter returns an iterator with the next item set to be None.
        .map(| i | i.to_string())
        // now, we get an item and return a string
        .collect();
        // and finally we collect the transformed items to another vector.
        how does this flow?
        This way:
        The collector, in this case the collect() method, is the one from the last map.
        The collector calls next to obtain the first item.
        The first thing the map (the one right above the .collect() does, is to call next on its original iterator, which is the one returned by the filter transformer.
        The first thing the filter does, by its turn, is to call next() on its original iterator, which is the one returned by the first map.
        The first thing the first map does is call next on its original iterator, which is the .iter() iterator from the vec of usizes.
        Right, we have every one calling next on its original iterator. The vec is not a transformer, it is a collection, so it returns its first item, which happens to be 1.
        The first map now applies the closure | i | i * 2, meaning that it returns an iterator with the next item having the value 2.
        The filter therefore gets one item with value 2 as its next item. It aplies the test in the closure | i | i <= 6 and the test passes, so that this very same item is returned as its iterator's next item.
        The last map gets the item with value 2 and applies the closure | i | i.to_string(), and returns an iterator containing "2" as its next value.
        The collector gets "2" and appends to its vector of strings.
        Then, the collector calls next() on the last map, which calls next() on the filter, which calls next() on the first map, which calls next() on the vector of usizes.
        The vector of usizes returns its next item, which has the value 2.
        The first map applies the closure | i | i * 2 and returns an item with value 4 on its iterator.
        The filter gets tthe item with value 4 and applies the test in the closure | i | i <= 6 and it passes the test, so the filter returns the item with value 4 as its iterator next item.
        The last map applies the closure | i | i.to_string() and returns an iterator with "4" as its next item.
        The collector gets "4" and appends it to its vector of strings.
        Then, the collector calls next() on the last map, which calls next() on the filter, which calls next() on the first map, which calls next() on the vector of usizes.
        The vector of usizes returns its next item, which has the value 3.
        The first map applies the closure | i | i * 2 and returns an item with value 6 on its iterator.
        The filter gets tthe item with value 6 and applies the test in the closure | i | i <= 6 and it passes the test, so the filter returns the item with value 6 as its iterator next item.
        The last map applies the closure | i | i.to_string() and returns an iterator with "6" as its next item.
        The collector gets "6" and appends it to its vector of strings.
        Then, the collector calls next() on the last map, which calls next() on the filter, which calls next() on the first map, which calls next() on the vector of usizes.
        The vector of usizes returns its next item, which has the value 4.
        The first map applies the closure | i | i * 2 and returns an item with value 8 on its iterator.
        The filter gets tthe item with value 8 and applies the test in the closure | i | i <= 6. It fails, so the filter calls the next item from its original iterator, the one returned by the first map.
        The first map calls next() on its original iterator, the iterator of the vector of usizes. It returns 5 and the map applies the closure | i | i * 2, so the map returns the next item to filter as 10.
        The filter gets tthe item with value 10 and applies the test in the closure | i | i <= 6. It fails, so the filter calls the next item from its original iterator, the one returned by the first map.
        The first map calls next() on its original iterator, the iterator of the vector of usizes. It returns 6 and the map applies the closure | i | i * 2, so the map returns the next item to filter as 12.
        The filter gets tthe item with value 12 and applies the test in the closure | i | i <= 6. It fails, so the filter calls the next item from its original iterator, the one returned by the first map.
        The first map calls next() on its original iterator, the iterator of the vector of usizes. It returns 7 and the map applies the closure | i | i * 2, so the map returns the next item to filter as 14.
        The filter gets tthe item with value 14 and applies the test in the closure | i | i <= 6. It fails, so the filter calls the next item from its original iterator, the one returned by the first map.
        The first map calls next() on its original iterator, the iterator of the vector of usizes. It returns None, because 7 was the last item on the usizes vector. Because it returns None, the first map also returns None, because the items have ended.
        The filter gets None and understands that items have ended, so it returns None.
        The last map gets None and understands that items have ended, so it returns None.
        The collector sees the next item as None, so it understands that its task is completed. It returns a vector with values ['"2", "4", "6"]
        At this time, you should have a good understanding on how transforming plays with iterators and how iterators play with collectors.
        Rust offers a huge quantity of transformers and collectors. You can look on collections documentation to see what the possibilities are.

        We will be using a flat_map. This transformer first gets an item and transforms it into a vector of items. The input itens and the output itens do not need to be of the same type, after all we are using a map.
        The only requirement is that one (single) input item must correspond to a vector of output itens. Obviously, as required by rust, output itens have to be all from the same type, though again this does not need to be the same type as the input type.
        But we are still not finished. The same transformer flatens the vectors of output items into a single vector of items.
        This means that the transformer:
        1. Calls next on the original item.
        2. Receives an item.
        3. Calls the closure on that item and gets back a vector of output items.
        4. Now, it will return all the items of these vector in sequence as the next items of its iterator.
        5. After returning all items from the vector obtained from the first item on the original iterator, when the next item is requested, steps 1 to 4 are repeated, so that another item is obtained from the original iterator, the closure is applied, a vector of items is obtained, and its items are returned in sequence as the next items on the flat_map iterator.
        6. When the original iterator returns none, the flat_map returns also None as its iterator's next item, which ends the whole processing.
    */
    let res = spec
        .item
        .iter()
        // base name is blank here because these are the items at the root level in the postman collection
        // notice that the convert_request_or_folder function takes an item and returns a vector of items
        .flat_map(|i| convert_request_or_folder("", i))
        .collect();
    handle_errors(&res);
    let http_requests = res
        .into_iter()
        .filter_map(|i| {
            if i.is_ok() {
                return Some(i.unwrap());
            }
            None
        })
        .collect();

    restclient::HttpRequests {
        requests: http_requests,
    }
}

fn handle_errors(res: &Vec<Result<restclient::HttpRequest, PostresError>>) {
    let errors: Vec<_> = res
        .iter()
        .filter_map(|i| {
            if let Err(e) = i {
                return Some(e);
            }
            None
        })
        .collect();
    if !errors.is_empty() {
        println!("Process terminated with errors:");
        for i in errors {
            println!("{i}");
        }
    }
}

fn convert_request_or_folder(
    basename: &str,
    item: &v2_1_0::Items,
) -> Vec<Result<restclient::HttpRequest, PostresError>> {
    let name = make_base_name(basename, item.name.as_ref().unwrap());
    if is_request(item) {
        // convert request and return a vec with it
        return vec![convert_request(&name, item)];
    }
    // process recursively the list of requests
    convert_folder(&name, &item.item)
}

fn convert_folder(
    name: &str,
    items: &Option<Vec<v2_1_0::Items>>,
) -> Vec<Result<restclient::HttpRequest, PostresError>> {
    items
        .as_ref()
        .unwrap()
        .iter()
        .flat_map(|i| convert_request_or_folder(&name, i))
        .collect()
}

fn is_request(item: &v2_1_0::Items) -> bool {
    // An object called items (plural) can perfectly signify a request (not a folder).
    // If it is a request, it iis a leaf in reqests tree
    if let None = item.item {
        return true;
    }
    matches!(item.item.as_ref(), Some(i) if i.is_empty())
}

fn convert_variables(content: &str) -> String {
    unimplemented!()
}

fn extract_path_variables(content: &str) -> String {
    let re_path_variables = Regex::new(r#"\/\:(.*?)(\/|$)"#).unwrap();
    let res = re_path_variables.replace_all(content, |caps: &Captures| {
        format!("/{{{{{}}}}}{}", &caps[1], &caps[2])
    });
    res.to_string()
}

/*
    see 009
*/
#[cfg(test)]
mod tests {
    use crate::postman::v2_1_0;

    use super::*;

    #[test]
    fn should_detect_as_request_when_list_of_requests_is_none() {
        let req = default_postman_items();
        assert_eq!(is_request(&req), true);
    }

    #[test]
    fn should_detect_as_request_when_list_of_requests_is_empty() {
        let mut req = default_postman_items();
        req.item = Some(vec![]);
        assert_eq!(is_request(&req), true);
    }

    #[test]
    fn should_not_detect_as_request_when_list_of_requests_is_not_empty() {
        let mut req = default_postman_items();
        req.item = Some(vec![default_postman_items()]);
        assert_eq!(is_request(&req), false);
    }

    pub fn default_postman_items() -> v2_1_0::Items {
        v2_1_0::Items {
            description: Default::default(),
            event: Default::default(),
            id: Default::default(),
            name: Default::default(),
            protocol_profile_behavior: Default::default(),
            request: Default::default(),
            response: Default::default(),
            variable: Default::default(),
            auth: Default::default(),
            item: Default::default(),
        }
    }

    pub fn default_postman_request_class() -> v2_1_0::RequestClass {
        let request = v2_1_0::RequestClass {
            auth: Default::default(),
            body: Default::default(),
            certificate: Default::default(),
            description: Default::default(),
            header: Default::default(),
            method: Default::default(),
            proxy: Default::default(),
            url: Default::default(),
        };
        request
    }
}
