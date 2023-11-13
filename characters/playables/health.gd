extends Node

var Heart = preload('res://objects/small-heart.tscn')

@onready var king: CharacterBody2D = get_owner()
@onready var sprite: Sprite2D = get_node("Sprite2D")
@onready var hearts: Node = get_node("Hearts")

func _process(delta):
	var heart = Heart.instantiate().get_node('AnimatedSprite2D')
	var diff = king.health - hearts.get_child_count()

	if diff > 0:
		var x = self.sprite.position.x - self.sprite.texture.get_width() / 2 + 7
		var y = self.sprite.position.y - 1

		heart.scale = self.sprite.scale

		for i in range(diff):
			heart.position = Vector2(x + hearts.get_child_count() * 22, y)
			self.hearts.add_child(heart.duplicate())

	elif diff < 0 and hearts.get_child_count() > 0:
		for i in range(abs(diff)):
			hearts.get_child(hearts.get_child_count() - 1).queue_free()
