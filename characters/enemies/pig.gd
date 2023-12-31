extends CharacterBody2D


const SPEED = 300.0
const JUMP_VELOCITY = -400.0

var gravity = ProjectSettings.get_setting("physics/2d/default_gravity")

func _ready():
	get_node("AnimatedSprite2D").play('Idle')


func _physics_process(delta):
	if not is_on_floor():
		velocity.y += gravity * delta

	move_and_slide()
