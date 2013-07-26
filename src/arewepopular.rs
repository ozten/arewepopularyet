

/* curl -G -v https://api.github.com/search/code --data-urlencode "q=language:javascript jquery"  -H "Accept: application/vnd.github.preview" */

extern mod search;

use search::search;

fn main() {
    // "websites"
    search("navigator.id.get OR navigator.id.request");

    //search("idproviders",
    //       "navigator.id.beginProvisioning or navigator.id.genKeyPair");
}