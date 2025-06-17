---@meta

---@class pawkit.logger
local logger = {}

---@param message string
function logger.print_to_console(message)
end

---@param message string
function logger.print_to_logfile(message)
end

---@param message string
function logger.info(message)
end

---@param message string
function logger.debug(message)
end

---@param message string
function logger.warn(message)
end

---@param message string
function logger.error(message)
end

---@param message string
function logger.fatal(message)
end

return logger
