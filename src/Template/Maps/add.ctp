<div class="maps form large-12 medium-8 columns content">
    <?= $this->Form->create($map, ['type'=>'file']) ?>
    <fieldset>
		<legend><?= __('Add Map') ?></legend>
		<?= $this->Form->input('Definition', ['type'=>'file', 'accept'=>'.map']); ?>
		<?= $this->Form->input('Image', ['type'=>'file', 'accept'=>'.rgb,.tga']); ?>
    </fieldset>
    <?= $this->Form->button(__('Submit')) ?>
    <?= $this->Form->end() ?>
</div>
