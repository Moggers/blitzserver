<div>
	<table id="posttable" cellpadding="0" cellspacing="0">
		<tbody>
			<?php foreach ($posts as $post): ?>
			<tr>
				<td><b><?=h($post->name)?></b>:<br /><?= h($post->comment) ?></td>
			</tr>
			<?php endforeach; ?>
		</tbody>
	</table>
	<div class="paginator">
		<ul class="pagination">
			<?= $this->Paginator->prev('< ' . __('previous')) ?>
			<?= $this->Paginator->numbers() ?>
			<?= $this->Paginator->next(__('next') . ' >') ?>
		</ul>
		<p><?= $this->Paginator->counter() ?></p>
	</div>
	<script>
		$(document).ready( function() {
			$('#postform').ajaxForm( function(res) {
				var data = JSON.parse(res);
				if( data.status == 0 ) {
					$('#posttable tr:first').before('<tr><td><b>' + data.name + '</b>:<br />'+data.comment+'</td></tr>');
				} else {
					alert( "Failure" );
				}
			});
		});
	</script>
	<?= $this->Form->create($newpost, ['id' => 'postform', 'url' => ['controller' => 'Posts', 'action' => 'add']]) ?>
	<fieldset style="padding:0px; margin:0px">
		<?php
			echo $this->Form->input('name', ['placeholder' => 'Name', 'label' => false, 'value' => ""]);
			echo $this->Form->input('comment', ['placeholder' => 'Comment', 'label' => false, 'value' => ""]);
			echo $this->Form->hidden('match_id' );
		?>
	</fieldset>
	</div>
	<?= $this->Form->button(__('Submit'), ['style' => 'padding: 5px 10px']) ?>
	<?= $this->Form->end() ?>
</div>
