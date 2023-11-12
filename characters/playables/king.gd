extends CharacterBody2D

const SPEED = 100.0
const JUMP_VELOCITY = -600.0
const MAX_HEALTH = 3;
const MAX_DIAMONS = 100;

# Get the gravity from the project settings to be synced with RigidBody nodes.
var gravity = ProjectSettings.get_setting("physics/2d/default_gravity")

var health = MAX_HEALTH;
var diamonds = 0;

@onready
var sprite = get_node("AnimatedSprite2D")

func _ready():
	sprite.play("Idle")

func _physics_process(delta):
	# Add the gravity.
	if not is_on_floor():
		velocity.y += gravity * delta

	# Hanlders
	handle_run()
	handle_jump()
	handle_attack()

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
	if (Input.is_action_just_pressed("ui_accept") or Input.is_joy_button_pressed(0, JOY_BUTTON_A)) and is_on_floor():
		velocity.y = JUMP_VELOCITY
	
	if velocity.y < 0:
		sprite.play("Jump")
	elif velocity.y > 0:
		sprite.play("Fall")
		
func handle_attack():
	if Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) or Input.is_joy_button_pressed(0, JOY_BUTTON_X):
		sprite.play("Attack")
		
func damange():
	health -= 1;
	get_node("Camera2D/CanvasLayer/Label").text = str(health)
	
