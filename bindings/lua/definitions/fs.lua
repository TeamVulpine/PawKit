---@meta

---@class pawkit.fs
local fs = {}

---@class VfsBuffer
local VfsBuffer = {}

---@return string
function VfsBuffer:read()
end

---@param bytes string
---@return VfsBuffer
function fs.buffer_from_bytes(bytes)
end

---@class VfsList
local VfsList = {}

---@param ext string
---@return VfsList
function VfsList:with_extension(ext)
end

---@class Vfs
local Vfs = {}

---@param dir string
---@return Vfs
function Vfs:subdirectory(dir)
end

---@param path string
---@return VfsBuffer
function Vfs:open(path)
end

---@return VfsList
function Vfs:list_subdirectories()
end

---@return VfsList
function Vfs:list_files()
end

---@return VfsList
function Vfs:list_files_recursive()
end

---@return Vfs
function fs.working()
end

---@param buf VfsBuffer
---@return Vfs
function fs.zip(buf)
end

return fs
