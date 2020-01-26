var holochain_connection = holochainclient.connect();

function hello() {
  holochain_connection.then(({ callZome, close }) => {
    callZome(
      "test-instance",
      "into_pieces",
      "hello_holo",
    )({ args: {} }).then(result => console.log(result));
  });
}
