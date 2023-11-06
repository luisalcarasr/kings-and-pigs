extends CharacterBody2D

const SPEED = 100.0
const JUMP_VELOCITY = -220.0

# Get the gravity from the project settings to be synced with RigidBody nodes.
var gravity = ProjectSettings.get_setting("physics/2d/default_gravity")

@onready
var sprite = get_node("AnimatedSprite2D")

func _ready():
	sprite.play("Idle")

func _physics_process(delta):
	# Add the gravity.
	if not is_on_floor():
		velocity.y += gravity * delta

	# Hanlders
	handle_jump()
	handle_run()

	move_and_slide()
	
func handle_run():
	var direction = Input.get_axis("ui_left", "ui_right")

	if direction > 0:
		sprite.flip_h = false
	elif direction < 0:
		sprite.flip_h = true
	
	if direction:
		velocity.x = direction * SPEED
	else:
		velocity.x = move_toward(velocity.x, 0, SPEED)
		
	if velocity.x != 0:
		sprite.play("Run")
	else:
		sprite.play("Idle")
		
func handle_jump():
	if Input.is_action_just_pressed("ui_accept") and is_on_floor():
		velocity.y = JUMP_VELOCITY
