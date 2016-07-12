<div style="">
	<div style="max-height:350px; overflow:auto">
		<table class="table table" style="overflow:auto" id="posttable" cellpadding="0" cellspacing="0">
			<tbody>
				<?php foreach ($posts as $post): ?>
				<tr>
					<td><b><?=h($post->name)?></b></td><td><?= h($post->comment) ?></td>
				</tr>
				<?php endforeach; ?>
			</tbody>
		</table>
	</div>
	<script>
		$(document).ready( function() {
			$('#postform').ajaxForm( function(res) {
				var data = JSON.parse(res);
				if( data.status == 0 ) {
					$('#posttable tr:first').before('<tr><td><b>' + data.name + '</td><td>'+data.comment+'</td></tr>');
				} else {
					alert( "Failure" );
				}
			});
		});
	</script>
	<?= $this->Form->create($newpost, ['id' => 'postform', 'url' => ['controller' => 'Posts', 'action' => 'add']]) ?>
	<fieldset style="padding:0px; margin:0px">
		<div class="input-group">
			<input type="text" class="form-control" placeholder="Name" name="name"></input>
			<span class="input-group-btn"><button type="submit" class="btn btn-default">Submit</button></span>
		</div>
		<textarea class="form-control custom-control" name="comment" rows="5"></textarea>
		<?= $this->Form->hidden('match_id' ); ?>
	</fieldset>
	<?= $this->Form->end() ?>
</div>
