<script type='text/javascript' >
	function swapImage(id, mid, size)
	{
		$( '#'+id + 'mapimage' )[0].src = '//blitzserver.net/img/maps/' + mid + '/thumb'+size+'.jpeg';
		if( size > 128 ) {
			$('#'+id+'mapimage').css('position', 'absolute');
			$('#'+id+'mapimage').css('z-index', 10);
		}
		else { 
			$('#'+id+'mapimage').css('position', 'relative');
			$('#'+id+'mapimage').css('z-index', 'auto');
		}
	}
	$(function() {
		$("#tabs").tabs();
	});
</script>
<ul style="padding:20px 0px" class="nav nav-pills">
	<li role="presentation" class="active"><a href="#progress" aria-controls="progress" role="tab" data-toggle="tab">In Progress</a></li>
	<li role="prsentation"><a href="#lobby" aria-controls="lobby" role="tab" data-toggle="tab">In Lobby</a></li>
	<li role="presentation"><a href="#finished" aria-controls="finished" role="tab" data-toggle="tab">Finished</a></li>
</ul>
<div class="tab-content">
	<div role="tabpanel" class="tab-pane active" id="progress">
		<table class="table" cellpadding="0" cellspacing="0">
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
				<td style="word-wrap:break-word"><?= $match->false_name ?></td>
				<td style="height:81px"><?= $match->has('map') ? $this->Html->link(
					$this->Html->image( '//blitzserver.net//img/maps/' . $match->map->id . '/thumb64.jpeg', [ 
						'id' => $match->id . 'mapimage', 
						'style' => 'min-width:64px; min-height:64px',
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
					<a class="btn btn-info btn-xs" href="/matches/view/<?=$match->id?>/"><span class="glyphicon glyphicon-wrench"></span></a>
				</td>
			</tr>
			<?php endforeach; ?>
		</tbody>
		</table>
	</div>
	<div role="tabpanel" class="tab-pane" id="lobby">
		<table class="table" cellpadding="0" cellspacing="0">
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
			<td style="word-wrap:break-word"><?= $match->false_name ?></td>
			<td style='height:81px'><?= $match->has('map') ? $this->Html->link(
				$this->Html->image( '//blitzserver.net/img/maps/' . $match->map->id . '/thumb64.jpeg', [ 
					'id' => $match->id . 'mapimage', 
					'style' => 'min-width:64px; min-height:64px',
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
					<a class="btn btn-info btn-xs" href="/matches/view/<?=$match->id?>/"><span class="glyphicon glyphicon-wrench"></span></a>
			</td>
		</tr>
		<?php endforeach; ?>
		</tbody>
		</table>
	</div>
	<div role="tabpanel" class="tab-pane" id="finished">
		<table class="table" cellpadding="0" cellspacing="0">
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
			<td><p style="max-width:200px; word-wrap:break-word"><?= $match->false_name ?></p></td>
			<td style='height:81px'><?= $match->has('map') ? $this->Html->link(
				$this->Html->image( '//blitzserver.net/img/maps/' . $match->map->id . '/thumb64.jpeg', [ 
					'id' => $match->id . 'mapimage', 
					'style' => 'min-width:64px; min-height:64px',
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
					<a class="btn btn-info btn-xs" href="/matches/view/<?=$match->id?>/"><span class="glyphicon glyphicon-wrench"></span></a>
			</td>
		</tr>
		<?php endforeach; ?>
		</tbody>
		</table>
	</div>
</div>
