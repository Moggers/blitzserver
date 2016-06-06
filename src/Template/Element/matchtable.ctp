<script type='text/javascript' >
	function swapImage(id, mid, size)
	{
		document.getElementById( id + 'mapimage' ).src = 'img/maps/' + mid + '/thumb'+size+'.jpeg';
	}
	$(function() {
		$("#tabs").tabs();
	});
</script>
<div id="tabs">
	<ul>
		<li><a href="#tabs-1">In Progress</a></li>
		<li><a href="#tabs-2">In Lobby</a></li>
		<li><a href="#tabs-3">FInished</a></li>
	</ul>
	<div id="tabs-1">
		<table cellpadding="0" cellspacing="0">
		<thead>
			<tr>
				<th><?= $this->Paginator->sort('name') ?></th>
				<th><?= $this->Paginator->sort('map_id') ?></th>
				<th><?= $this->Paginator->sort('age') ?></th>
				<th><?= $this->Paginator->sort('address') ?></th>
				<th><?= $this->Paginator->sort('status') ?></th>
				<th><?= $this->Paginator->sort('thrones') ?></th>
				<th><?= $this->Paginator->sort('players') ?></th>
				<th><?= $this->Paginator->sort('turn') ?></th>
				<th><?= $this->Paginator->sort('action') ?></th>
			</tr>
		</thead>
		<tbody>
		<?php foreach ($progressmatches as $match): ?>
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
			<td><?= $match->address ?></td>
			<td><?= $match->status_string ?></td>
			<td><?= $match->thrones ?></td>
			<td><?= $match->player_count ?></td>
			<td><?= $match->turn->tn ?></td>
			<td class="actions">
				<?= $this->Html->link(__('Unstart Match'), ['action' => 'unstart', $match->id]) ?> <br /> 
				<?= $this->Html->link(__('End Match'), ['action' => 'finish', $match->id]) ?> <br />
				<?= $this->Html->link(__('Details'), ['action' => 'view', $match->id]) ?>
			</td>
		</tr>
		<?php endforeach; ?>
		</tbody>
		</table>
	</div>
	<div id="tabs-2">
		<table cellpadding="0" cellspacing="0">
		<thead>
			<tr>
				<th><?= $this->Paginator->sort('name') ?></th>
				<th><?= $this->Paginator->sort('map_id') ?></th>
				<th><?= $this->Paginator->sort('age') ?></th>
				<th><?= $this->Paginator->sort('address') ?></th>
				<th><?= $this->Paginator->sort('status') ?></th>
				<th><?= $this->Paginator->sort('thrones') ?></th>
				<th><?= $this->Paginator->sort('players') ?></th>
				<th><?= $this->Paginator->sort('action') ?></th>
			</tr>
		</thead>
		<tbody>
		<?php foreach ($lobbymatches as $match): ?>
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
			<td><?= $match->address ?></td>
			<td><?= $match->status_string ?></td>
			<td><?= $match->thrones ?></td>
			<td><?= $match->player_count ?></td>
			<td class="actions">
				<?= $this->Html->link(__('Details'), ['action' => 'view', $match->id]) ?> <br />
				<?= $this->Html->link(__('Cancel Match'), ['action' => 'destroy', $match->id]) ?> 
			</td>
		</tr>
		<?php endforeach; ?>
		</tbody>
		</table>
	</div>
	<div id="tabs-3">
		<table cellpadding="0" cellspacing="0">
		<thead>
			<tr>
				<th><?= $this->Paginator->sort('name') ?></th>
				<th><?= $this->Paginator->sort('map_id') ?></th>
				<th><?= $this->Paginator->sort('age') ?></th>
				<th><?= $this->Paginator->sort('status') ?></th>
				<th><?= $this->Paginator->sort('thrones') ?></th>
				<th><?= $this->Paginator->sort('players') ?></th>
				<th><?= $this->Paginator->sort('turn') ?></th>
				<th><?= $this->Paginator->sort('action') ?></th>
			</tr>
		</thead>
		<tbody>
		<?php foreach ($finishedmatches as $match): ?>
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
			<td><?= $match->status_string ?></td>
			<td><?= $match->thrones ?></td>
			<td><?= $match->player_count ?></td>
			<td><?= $match->turn->tn ?></td>
			<td class="actions">
				<?= $this->Html->link(__('Details'), ['action' => 'view', $match->id]) ?>
			</td>
		</tr>
		<?php endforeach; ?>
		</tbody>
		</table>
	</div>
</div>
