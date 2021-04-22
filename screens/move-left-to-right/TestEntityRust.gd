extends Polygon2D

var velocity: Vector2 = Vector2(100.0, 0.0)

var entity_id: int

###############################################################################
# Builtin functions                                                           #
###############################################################################

func _ready() -> void:
	GameManager.entity_counter += 1
	entity_id = GameManager.entity_counter
	
	GameManager.ecs.register_entity(entity_id, global_position, velocity)
	
	$VisibilityNotifier2D.connect("screen_exited", self, "_on_exit_screen")

func _physics_process(delta: float) -> void:
	global_position = GameManager.ecs.read_data(entity_id)

func _exit_tree() -> void:
	GameManager.ecs.unregister_entity(entity_id)

###############################################################################
# Connections                                                                 #
###############################################################################

func _on_exit_screen() -> void:
	GameManager.ecs.unregister_entity(entity_id)
	queue_free()

###############################################################################
# Private functions                                                           #
###############################################################################

###############################################################################
# Public functions                                                            #
###############################################################################


