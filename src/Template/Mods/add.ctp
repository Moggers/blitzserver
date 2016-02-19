<div class="mods form large-12 medium-8 columns content">
    <?= $this->Form->create($mod, ['type'=>'file']) ?>
    <fieldset>
        <legend><?= __('Add Mod') ?></legend>
		<?= $this->Form->input('Archive', [ 'label' => 'Zip Archive', 'type'=>'file', 'accept'=>'.zip,.7z,.rar']); ?>
    <?= $this->Form->button(__('Submit')) ?>
    <?= $this->Form->end() ?>
</div>
