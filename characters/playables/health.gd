extends Sprite2D

var Heart = preload('res://objects/small-heart.tscn')

@onready var king = get_owner()
@onready var hearts = get_parent().get_node('Hearts')

func _process(delta):
	var diff = king.health - hearts.get_child_count()
	if diff > 0:
		
		
func clear():
	var children = hearts.get_children()
	for child in children:
		child.queue_free()

func draw(health: int):
	var heart = Heart.instantiate().get_node('AnimatedSprite2D')
	
	var x = self.position.x - self.texture.get_width() / 2 + 7
	var y = self.position.y - 1

	heart.scale = self.scale
	for i in range(health):
		heart.position = Vector2(x, y)
		hearts.add_child(heart.duplicate())
		x += 22

func _ready():
	draw(king.health)
