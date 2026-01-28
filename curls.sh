function get() {
  curl -X GET http://127.0.0.1:8080/usages/3
}
function get_all() {
  curl -X GET http://127.0.0.1:8080/usages
}
function new() {
  curl -X POST \
       -H 'Content-Type: application/json' \
       -d '{"Plant":1,"Pot":1,"Planted":"2016-01-01 01:00:00"}' \
       http://127.0.0.1:8080/usages
}
function update() {
  curl -X PUT \
       -H 'Content-Type: application/json' \
       -d '{"Plant":7}' \
       http://127.0.0.1:8080/usages/3
}
function delete() {
  curl -X DELETE http://127.0.0.1:8080/usages/10
}
