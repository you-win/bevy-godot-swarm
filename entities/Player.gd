extends Polygon2D

var entity_id: int

###############################################################################
# Builtin functions                                                           #
###############################################################################

func _ready() -> void:
	GameManager.entity_counter += 1
	entity_id = GameManager.entity_counter
	
	GameManager.ecs.register_player(entity_id, global_position)

func _physics_process(delta: float) -> void:
	self.global_position = GameManager.ecs.read_data(entity_id)

func _exit_tree():
	GameManager.ecs.unregister_entity(entity_id)

###############################################################################
# Connections                                                                 #
###############################################################################

###############################################################################
# Private functions                                                           #
###############################################################################

###############################################################################
# Public functions                                                            #
###############################################################################


