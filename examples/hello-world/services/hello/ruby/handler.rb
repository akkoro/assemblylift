require 'asml'
require 'base64'
require 'json'

def main(input)
    # TODO implement your function code here!
    Asml.log("Received function input: " + input.to_s)
    Asml.success(JSON.generate("Hello world!"))
end

main(JSON.parse(Asml.get_function_input()))
