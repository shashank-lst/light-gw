local cjson = require "cjson.safe"
local ljsonschema = require "resty.ljsonschema"

local OpenApiValidator = {}
OpenApiValidator.__index = OpenApiValidator

-- Constructor
function OpenApiValidator.new(spec_path)
  local self = setmetatable({}, OpenApiValidator)

  -- Load the OpenAPI spec
  local file, err = io.open(spec_path, "r")
  if not file then
    error("Failed to open OpenAPI spec file: " .. spec_path .. ": " .. (err or "unknown error"))
  end

  local spec_content = file:read("*a")
  file:close()

  local spec, err = cjson.decode(spec_content)
  if not spec then
    error("Failed to parse OpenAPI spec: " .. (err or "unknown error"))
  end

  self.spec = spec
  return self
end

-- Validate the request body against the OpenAPI spec
function OpenApiValidator:validate_request(path, method, body)
  local path_spec = self.spec.paths[path]
  if not path_spec then
    return false, "Path not defined in OpenAPI spec"
  end

  local method_spec = path_spec[method:lower()]
  if not method_spec then
    return false, "Method not defined for path in OpenAPI spec"
  end

  if method_spec.requestBody then
    local content = method_spec.requestBody.content
    if content and content["application/json"] and content["application/json"].schema then
      local schema = content["application/json"].schema
      local validator = ljsonschema.new(schema)
      local ok, err = validator:validate(body)
      if not ok then
        return false, err
      end
    end
  end

  return true
end

return OpenApiValidator
