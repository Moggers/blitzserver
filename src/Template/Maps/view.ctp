<div class="maps view large-12 medium-12 columns content">
    <h3><?= h($map->name) ?></h3>
	<div class="wrapper">
		<div class="maptable">
			<table class="vertical-table" style="background:#fafafa">
				<table class="vertical-table" style="background:#fafafa">
					<tr>
						<th><?= __('Description') ?></th>
						<td><?= h($map->description) ?></td>
					</tr>
					<tr>
						<th><?= __('Provinces(Sea Provinces)')?> </th>
						<td><?= h($map->prov.'('.$map->seaprov) .')' ?> </td>
					</tr>
				</table>
			</table>
		</div>
		<div class="mappreview">
			<td><?=$this->Html->image( 'maps/' . $map->id . '/thumb1024.jpeg', ['alt' => 'CakePHP' ]) ?></td>
		</div>
	</div>
	</table>
</div>
