<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('List Matches'), ['action' => 'index']) ?></li>
        <li><?= $this->Html->link(__('List Maps'), ['controller' => 'Maps', 'action' => 'index']) ?></li>
        <li><?= $this->Html->link(__('New Map'), ['controller' => 'Maps', 'action' => 'add']) ?></li>
    </ul>
</nav>
<div class="matches form large-9 medium-8 columns content">
    <?= $this->Form->create($match) ?>
    <fieldset>
        <legend><?= __('Add Match') ?></legend>
        <?php
			?>
			<h5>Important Stuff</h5>
			<?php
			echo $this->Form->input('name');
            echo $this->Form->input('map_id', ['options' => $maps]);
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
				'options' => array(1 => "Very Easy", 2 => "Easy", 3 => "Normal", 4 => "Hard", 5 => "Very Hard"),
				'value' => 3,
				'label' => 'Research Difficulty'));
			echo $this->Form->input( 'renaming', array(
				'label' => 'Commander Renaming',
				'type' => 'checkbox'  ));
        ?>
    </fieldset>
    <?= $this->Form->button(__('Submit')) ?>
    <?= $this->Form->end() ?>
</div>
