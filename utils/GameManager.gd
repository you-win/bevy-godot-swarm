extends Node

signal message_logged(message)

var sdu: SaveDataUtil = SaveDataUtil.new()

var current_save_data: Dictionary

var ecs = load("res://utils/ECS.gdns").new()

###############################################################################
# Builtin functions                                                           #
###############################################################################

###############################################################################
# Connections                                                                 #
###############################################################################

###############################################################################
# Private functions                                                           #
###############################################################################

###############################################################################
# Public functions                                                            #
###############################################################################

func log_message(message: String, is_error: bool = false) -> void:
	if is_error:
		message = "[ERROR] %s" % message
		assert(false, message)
	print(message)
	emit_signal("message_logged", message)
