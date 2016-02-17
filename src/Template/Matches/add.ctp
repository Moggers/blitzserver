<script type="text/javascript">
	$(document).ready( function() {
		$('#matchcreate').ajaxForm( function(res) {
			var data = JSON.parse( res );
			if( data.status == 1 ) {
				window.location = '/matches/view/'+data.id;
			} else {
				window.location.hash = "modal";
			}
		});
		$('#modtable tr').on('mouseover', function(e) {
			if( e.delegateTarget.id.substring(0,3) == 'mod' ) {
				if( $('#mod-'+e.delegateTarget.id.substring(4,6)).prop( 'checked' ) == false ) {
					e.delegateTarget.style.background = '#f0f0ff';
				}
			}
		});
		$('#modtable tr').on('mouseout', function(e) {
			if( e.delegateTarget.id.substring(0,3) == 'mod' ) {
				if( $('#mod-'+e.delegateTarget.id.substring(4,6)).prop( 'checked' ) == false ) {
					e.delegateTarget.style.background = '#ffffff';
				}
			}
		});
		$('#modtable tr').on('click', function(e) {
			if( e.delegateTarget.id.substring(0,3) == 'mod' ) {
				if( $('#mod-'+e.delegateTarget.id.substring(4,6)).prop( 'checked' ) == true ) {
					$('#mod-'+e.delegateTarget.id.substring(4,6)).prop( 'checked', false );
					e.delegateTarget.style.background = '#f0f0ff';
				} else { 
					$('#mod-'+e.delegateTarget.id.substring(4,6)).prop( 'checked', true );
					e.delegateTarget.style.background = '#babaff';
				}
			}
		});
	});
</script>

<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
    </ul>
</nav>
<div class="matches form large-9 medium-8 columns content">
    <?= $this->Form->create($match, ['id' => 'matchcreate']) ?>
    <fieldset>
        <legend><?= __('Add Match') ?></legend>
        <?php
			?>
			<h5>Important Stuff</h5>
			<?php
			echo $this->Form->input('name');
            echo $this->Form->input('map_id', ['options' => $maps->where(['hide' => 0])]);
            echo $this->Form->input('age', array(
				'options' => array( 1 => 'Early', 2 => 'Middle', 3 => 'Late'),
				'value' => 1 ));
			?>
			<h5>Thrones</h5>
			<?php
			echo $this->Form->input('tone',
				array('label' => 'T1 Thrones', 'default' => 5 ));
			echo $this->Form->input('ttwo',
				array('label' => 'T2 Thrones', 'default' => 0 ));
			echo $this->Form->input('tthree',
				array('label' => 'T3 Thrones', 'default' => 0 ));
			echo $this->Form->input('points',
				array('label' => 'Points To Win', 'default' => 5 ));
			?>
			<h5>Misc</h5>
			<?php
			echo $this->Form->input( 'research_diff', array(
				'options' => array(-1 => "Very Easy", 0 => "Easy", 1 => "Normal", 2 => "Hard", 3 => "Very Hard"),
				'value' => 1,
				'label' => 'Research Difficulty'));
			echo $this->Form->input( 'renaming', array(
				'label' => 'Commander Renaming',
				'type' => 'checkbox'  ));
			echo "Until I fix the no thrones bug, this needs to be enabled, and the game needs to be started using the Start Game button from inside the client once everyone has joined. Starting the match from this interface will cause there to be NO THRONES";
			echo $this->Form->input( 'clientstart', array(
				'label' => 'Clients Can Start Game',
				'type' => 'checkbox'  ));
			?>
			<h5>Mods</h5>
			<?= $this->Form->input( 'Mod', ['multiple' => 'checkbox', 'id' => 'hiddenmods' ] ); ?>
			<?= $this->Html->link( 'Select Mods', '#modselect' ); ?>
		<div class="remodal" data-remodal-id="modselect">
			<?= $this->element( 'modtable', array( 'mods' => $modsfull )); ?>
		</div>
    </fieldset>
    <?= $this->Form->button(__('Submit')) ?>
    <?= $this->Form->end() ?>
</div>
