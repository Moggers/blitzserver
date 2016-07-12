<script type='text/javascript' >
	function swapImage(id, mid, size)
	{
		$( '#'+id + 'mapimage' )[0].src = '/img/maps/' + mid + '/thumb'+size+'.jpeg';
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
<ul class="nav nav-tabs">
	<li role="presentation" class="active"><a href="#progress" aria-controls="progress" role="tab" data-toggle="tab">In Progress</a></li>
	<li role="prsentation"><a href="#lobby" aria-controls="lobby" role="tab" data-toggle="tab">In Lobby</a></li>
	<li role="presentation"><a href="#finished" aria-controls="finished" role="tab" data-toggle="tab">Finished</a></li>
</ul>
<div class="tab-content">
	<div role="tabpanel" class="tab-pane active" id="progress">
		<table class="table table-bordered table-hover" cellpadding="0" cellspacing="0">
		<thead>
			<tr>
				<th style="width:200px">Name</th>
				<th>Age</th>
				<th>Address</th>
				<th>Thrones</th>
				<th>Players</th>
				<th>Turn</th>
			</tr>
		</thead>
		<tbody>
			<?php foreach ($progressmatches as $match): ?>
			<tr style="cursor:pointer" onclick="window.location = '/matches/view/<?=$match->id?>'">
				<td><?= $this->Text->truncate($match->false_name, 40); ?></td>
				<td><?= $match::ages( $match->age ) ?></td>
				<td><?= $match->address ?></td>
				<td><?= $match->thrones ?></td>
				<td><?= $match->player_count ?></td>
				<td><?= $match->turn->tn ?></td>
			</tr>
			<?php endforeach; ?>
		</tbody>
		</table>
	</div>
	<div role="tabpanel" class="tab-pane" id="lobby">
		<table class="table table-bordered table-hover" cellpadding="0" cellspacing="0">
		<thead>
			<tr>
				<th style="width:200px">Name</th>
				<th>Age</th>
				<th>Address</th>
				<th>Thrones</th>
				<th>Players</th>
			</tr>
		</thead>
		<tbody>
		<?php foreach ($lobbymatches as $match): ?>
		<tr style="cursor:pointer" onclick="window.location = '/matches/view/<?=$match->id?>'">
			<td><?= $this->Text->truncate($match->false_name, 40); ?></td>
			<td><?= $match::ages( $match->age ) ?></td>
			<td><?= $match->address ?></td>
			<td><?= $match->thrones ?></td>
			<td><?= $match->player_count ?></td>
		</tr>
		<?php endforeach; ?>
		</tbody>
		</table>
	</div>
	<div role="tabpanel" class="tab-pane" id="finished">
		<table class="table table-bordered table-hover" cellpadding="0" cellspacing="0">
		<thead>
			<tr>
				<th style=" width:200px">Name</th>
				<th>Age</th>
				<th>Address</th>
				<th>Thrones</th>
				<th>Players</th>
			</tr>
		</thead>
		<tbody>
		<?php foreach ($finishedmatches as $match): ?>
		<tr style="cursor:pointer" onclick="window.location = '/matches/view/<?=$match->id?>'">
			<td><?= $this->Text->truncate($match->false_name, 40); ?></td>
			<td><?= $match::ages( $match->age ) ?></td>
			<td><?= $match->thrones ?></td>
			<td><?= $match->player_count ?></td>
			<td><?= $match->turn->tn ?></td>
		</tr>
		<?php endforeach; ?>
		</tbody>
		</table>
	</div>
</div>
