extends Node2D

###############################################################################
# Builtin functions                                                           #
###############################################################################

func _ready() -> void:
	pass

func _physics_process(delta: float) -> void:
	GameManager.ecs.step(delta)

###############################################################################
# Connections                                                                 #
###############################################################################

###############################################################################
# Private functions                                                           #
###############################################################################

###############################################################################
# Public functions                                                            #
###############################################################################


