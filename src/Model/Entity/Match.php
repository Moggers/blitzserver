<?php
namespace App\Model\Entity;

use Cake\ORM\Entity;

/**
 * Match Entity.
 *
 * @property int $id
 * @property int $map_id
 * @property \App\Model\Entity\Map $map
 * @property int $age
 */
class Match extends Entity
{

    /**
     * Fields that can be mass assigned using newEntity() or patchEntity().
     *
     * Note that when '*' is set to true, this allows all unspecified fields to
     * be mass assigned. For security purposes, it is advised to set '*' to false
     * (or remove it), and explicitly make individual fields accessible as needed.
     *
     * @var array
     */
    protected $_accessible = [
        '*' => true,
        'id' => false,
    ];

	public static function enum( $value, $options, $default = '' ) {
		if( $value !== null ) {
			if( array_key_Exists( $value, $options )) {
				return $options[$value];
			}
		return $default;
		}
		return $options;
	}

	public static function statuses( $value = null )
	{
		$options = [
			self::STATUS_DELETED => __('Marked for Deletion', true ),
			self::STATUS_NEW => __('Pending', true ),
			self::STATUS_LOBBY => __('In Lobby', true ),
			self::STATUS_STARTED => __('Starting', true ),
			self::STATUS_RUNNING => __('Running', true ),
			self::STATUS_COUNTDOWN_5 => __('Starting in 5 seconds', true ),
			self::STATUS_COUNTDOWN_10 => __('Starting in 10 seconds', true ),
			self::STATUS_COUNTDOWN_15 => __('Starting in 15 seconds', true ),
			self::STATUS_CRIT_FAILURE => __('Nagot gick fel', true)];
		return Match::enum( $value, $options );
	}

	public static function ages( $value = null )
	{
		$options = [
			self::AGE_EARLY => __('Early', true ),
			self::AGE_MIDDLE => __('Middle', true ),
			self::AGE_LATE => __('Late', true ) ];

		return Match::enum( $value, $options );
	}


	public static function races( $value = null )
	{
		$options = [
			self::RACE_EARLY_CTIS => __('EA C\'tis, Lizard Kings')];
		return Match::enum( $value, $options );
	}
	public static function getRaces( $value = null )
	{
		$ii = 0;
		$raceset = array();
		for( $ii = 0; $ii < 32; $ii++ ) {
			if( ($value >> $ii ) & 1 == 1 )
			{
				array_push( $raceset, match::races( $ii) );
			}
		}
		return $raceset;
	}

	const STATUS_DELETED = -1;
	const STATUS_NEW = 0;
	const STATUS_LOBBY = 1;
	const STATUS_STARTED = 2;
	const STATUS_RUNNING = 3;
	const STATUS_COUNTDOWN_5 = 11;
	const STATUS_COUNTDOWN_10 = 12;
	const STATUS_COUNTDOWN_15 = 13;
	const STATUS_CRIT_FAILURE = 99;

	const AGE_EARLY = 1;
	const AGE_MIDDLE = 2;
	const AGE_LATE = 3;

	const RACE_EARLY_CTIS = 15;
}
