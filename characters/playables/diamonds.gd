extends Node

@onready var label = get_node("Label")
@onready var sprite = get_node("AnimatedSprite2D")

# Called when the node enters the scene tree for the first time.
func _ready():
	sprite.play('Idle')


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	label.text = 'x ' + str(get_owner().diamonds)
