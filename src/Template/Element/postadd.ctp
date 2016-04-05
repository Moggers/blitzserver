<script>
	$(document).ready( function() {
		$('#postform').ajaxForm( function(res) {
			console.log('ayy lmao' );
		});
	});
</script>
<?= $this->Form->create($post, ['id' => 'postform', 'url' => ['controller' => 'Posts', 'action' => 'add']]) ?>
<fieldset style="padding:0px; margin:0px">
	<?php
		echo $this->Form->input('name', ['placeholder' => 'Name', 'label' => false]);
		echo $this->Form->input('comment', ['placeholder' => 'Comment', 'label' => false]);
		echo $this->Form->hidden('match_id' );
	?>
</fieldset>
</div>
<?= $this->Form->button(__('Submit'), ['style' => 'padding: 5px 10px']) ?>
<?= $this->Form->end() ?>
