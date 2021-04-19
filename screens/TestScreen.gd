extends Node2D

###############################################################################
# Builtin functions                                                           #
###############################################################################

func _ready() -> void:
	pass

func _physics_process(delta: float) -> void:
	GameManager.ecs.read_input()
	GameManager.ecs.step(delta)

func _unhandled_input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_accept"):
		print(GameManager.ecs.read_data("Player"))
	if event.is_action_pressed("ui_cancel"):
		GameManager.ecs.unregister_entity_deferred($Entities/Player.name)
		
		$Entities/Player.queue_free()

###############################################################################
# Connections                                                                 #
###############################################################################

###############################################################################
# Private functions                                                           #
###############################################################################

###############################################################################
# Public functions                                                            #
###############################################################################


