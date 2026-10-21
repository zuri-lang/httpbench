require 'json'

app = lambda do |env|
  path = env['PATH_INFO']
  method = env['REQUEST_METHOD']

  if path == '/health'
    [200, {'content-type' => 'text/plain'}, ['ok']]
  elsif path == '/plaintext'
    [200, {'content-type' => 'text/plain', 'content-length' => '13'}, ['Hello, World!']]
  elsif path == '/json'
    body = '{"message":"Hello, World!"}'
    [200, {'content-type' => 'application/json', 'content-length' => body.bytesize.to_s}, [body]]
  elsif path == '/echo' && method == 'POST'
    input = env['rack.input'].read
    [200, {'content-type' => 'application/octet-stream', 'content-length' => input.bytesize.to_s}, [input]]
  else
    [404, {'content-type' => 'text/plain'}, ['Not Found']]
  end
end

run app

