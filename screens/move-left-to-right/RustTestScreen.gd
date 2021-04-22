extends Node2D

const SPAWNER: Resource = preload("res://screens/move-left-to-right/Spawner.tscn")
const ENTITY: Resource = preload("res://screens/move-left-to-right/TestEntityRust.tscn")

###############################################################################
# Builtin functions                                                           #
###############################################################################

func _ready() -> void:
	var screen_size: Vector2 = get_viewport().size
	
	for i in int(screen_size.y / 50):
		var spawner: Node2D = SPAWNER.instance()
		
		spawner.entity = ENTITY
		call_deferred("add_child", spawner)
		
		spawner.position.y = i * 50

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


