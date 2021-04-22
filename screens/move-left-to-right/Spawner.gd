extends Position2D

const DURATION: float = 0.5

export var entity: Resource

onready var timer: Timer = $Timer

###############################################################################
# Builtin functions                                                           #
###############################################################################

func _ready() -> void:
	timer.connect("timeout", self, "_on_timeout")
	timer.start(DURATION)

###############################################################################
# Connections                                                                 #
###############################################################################

func _on_timeout() -> void:
	var entity_instance: Node2D = entity.instance()
	entity_instance.global_position = self.global_position
	get_parent().call_deferred("add_child", entity_instance)
	
	timer.start(DURATION)

###############################################################################
# Private functions                                                           #
###############################################################################

###############################################################################
# Public functions                                                            #
###############################################################################


