local BasePlugin = require "kong.plugins.base_plugin"
local OpenApiValidator = require "openapi_validator"  -- Update the path as needed
local cjson = require "cjson.safe"

local MyOpenApiPlugin = BasePlugin:extend()

MyOpenApiPlugin.VERSION = "0.1.0"
MyOpenApiPlugin.PRIORITY = 10

function MyOpenApiPlugin:new()
  MyOpenApiPlugin.super.new(self, "my-openapi-plugin")
end

function MyOpenApiPlugin:access(conf)
  MyOpenApiPlugin.super.access(self)
  
  -- Initialize the OpenAPI validator
  local validator = OpenApiValidator.new(conf.spec_path)
  
  -- Retrieve request details
  local request_path = kong.request.get_path()
  local request_method = kong.request.get_method()
  local request_body, err = kong.request.get_body()

  if not request_body then
    kong.log.err("Failed to get request body: ", err)
    return kong.response.exit(400, { message = "Invalid request body" })
  end
  
  -- Validate the request
  local ok, validation_err = validator:validate_request(request_path, request_method, request_body)
  if not ok then
    kong.log.err("Request validation failed: ", validation_err)
    return kong.response.exit(400, { message = "Bad Request", error = validation_err })
  end
end

return MyOpenApiPlugin
