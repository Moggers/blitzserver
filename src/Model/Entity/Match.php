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
			self::STATUS_RUNNING_C => __('Running', true ),
			self::STATUS_COUNTDOWN_5 => __('Starting in 5 seconds', true ),
			self::STATUS_COUNTDOWN_10 => __('Starting in 10 seconds', true ),
			self::STATUS_COUNTDOWN_15 => __('Starting in 15 seconds', true )];
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

	const STATUS_DELETED = -1;
	const STATUS_NEW = 0;
	const STATUS_LOBBY = 1;
	const STATUS_STARTED = 2;
	const STATUS_RUNNING = 3;
	const STATUS_RUNNING_C = 10;
	const STATUS_COUNTDOWN_5 = 11;
	const STATUS_COUNTDOWN_10 = 12;
	const STATUS_COUNTDOWN_15 = 13;

	const AGE_EARLY = 1;
	const AGE_MIDDLE = 2;
	const AGE_LATE = 3;
}
