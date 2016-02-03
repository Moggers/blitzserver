<script type='text/javascript' >
	function swapImage(id, mid, size)
	{
		document.getElementById( id + 'mapimage' ).src = 'img/maps/' + mid + '/thumb'+size+'.jpeg';
	}
</script>
<?php foreach ($matches as $match): ?>
<tr>
	<td><?= $match->false_name ?></td>
	<td><?= $match->has('map') ? $this->Html->link(
		$this->Html->image( 'maps/' . $match->map->id . '/thumb64.jpeg', [ 
			'id' => $match->id . 'mapimage', 
			'onmouseover' => 'swapImage('.$match->id.','.$match->map->id.',256'.')', 
			'onmouseout' =>  'swapImage('.$match->id.','.$match->map->id.',64'.')',
			'alt' => 'CakePHP' ]), 
		['controller' => 'Maps', 'action' => 'view', $match->map->id], 
		['escape' => false]) : '' ?> </td>
	<td><?= $match::ages( $match->age ) ?></td>
	<td><?= $match->safe_port ?></td>
	<td><?= $match->status_string ?></td>
	<td><?= $match->thrones ?></td>
	<td class="actions">
		<?php if ($match->status !== -1): ?>
			<?php if ($match->status < 2 ): ?>
				<?= $this->Html->link(__('Start Game'), ['action' => 'start', $match->id]) ?> <br />
			<?php endif; ?>
			<?= $this->Html->link(__('KILL THE GAME'), ['action' => 'destroy', $match->id]) ?> <br />
			<?= $this->Html->link(__('Details'), ['action' => 'view', $match->id]) ?>
		<?php endif; ?>
	</td>
</tr>
<?php endforeach; ?>
