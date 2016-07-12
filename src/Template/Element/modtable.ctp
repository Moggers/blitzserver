<table cellpadding="0" cellspacing="0" id='modtable' class="table table-bordered table-hover">
	<thead>
		<tr>
			<th style="width:350px">Name</th>
			<th style="width:128px">Icon</th>
			<th>Version</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody class="overflow-ellipsis">
		<?php foreach ($mods as $mod): ?>
		<tr class="modpane" id=<?= 'mod_'.$mod->id ?>>
			<td><?= h($mod->name) ?></td>
			<td style="padding:0px">
				<div style="width:144px; height:36px; background-size:144px 36px; background-image:url('/img/mods/<?=$mod->id?>/thumb64.jpeg'), url('/img/noimage.png')"></div>
			</td>
			<td><?= $mod->version ?></td>
			<td style="cellspacing='0'; cellpadding='0';"><div class="overflow-ellipsis"><?= h($mod->description) ?></div></td>
		</tr>
		<?php endforeach; ?>
	</tbody>
</table>
