local cjson = require "cjson.safe"

local function resolve_references(spec)
  local function resolve_ref(ref, base)
    local parts = ref:match("#/(.*)"):gmatch("[^/]+")
    local obj = base
    for part in parts do
      obj = obj[part]
      if not obj then
        return nil, "Reference not found: " .. ref
      end
    end
    return obj
  end

  local function resolve_schema(schema, base)
    if schema and schema["$ref"] then
      local ref = schema["$ref"]
      local resolved_schema, err = resolve_ref(ref, base)
      if not resolved_schema then
        return nil, err
      end
      return resolve_schema(resolved_schema, base)
    end
    return schema
  end

  local function resolve_all_schemas(spec)
    if spec.components and spec.components.schemas then
      for name, schema in pairs(spec.components.schemas) do
        spec.components.schemas[name] = resolve_schema(schema, spec.components.schemas)
      end
    end
  end

  resolve_all_schemas(spec)
  return spec
end

-- Load and resolve references in the OpenAPI specification
local function load_openapi_schema(spec_path)
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

  spec = resolve_references(spec)
  return spec
end
