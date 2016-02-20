-- MySQL dump 10.16  Distrib 10.1.9-MariaDB, for Linux (x86_64)
--
-- Host: localhost    Database: blitztest
-- ------------------------------------------------------
-- Server version	10.1.9-MariaDB-log

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `cake_admin_phinxlog`
--

DROP TABLE IF EXISTS `cake_admin_phinxlog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `cake_admin_phinxlog` (
  `version` bigint(20) NOT NULL,
  `start_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `end_time` timestamp NOT NULL DEFAULT '0000-00-00 00:00:00',
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `cake_admin_phinxlog`
--

LOCK TABLES `cake_admin_phinxlog` WRITE;
/*!40000 ALTER TABLE `cake_admin_phinxlog` DISABLE KEYS */;
INSERT INTO `cake_admin_phinxlog` VALUES (20150611195820,'2015-12-06 21:35:28','2015-12-06 21:35:28');
/*!40000 ALTER TABLE `cake_admin_phinxlog` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `maps`
--

DROP TABLE IF EXISTS `maps`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `maps` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `imagepath` varchar(255) DEFAULT NULL,
  `name` varchar(255) DEFAULT NULL,
  `mappath` varchar(255) DEFAULT NULL,
  `description` varchar(255) DEFAULT NULL,
  `prov` int(11) DEFAULT NULL,
  `seaprov` int(11) DEFAULT NULL,
  `vwrap` tinyint(1) DEFAULT NULL,
  `hwrap` tinyint(1) DEFAULT NULL,
  `hide` int(11) DEFAULT NULL,
  `crc32` int(10) unsigned DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=33 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `maps`
--

LOCK TABLES `maps` WRITE;
/*!40000 ALTER TABLE `maps` DISABLE KEYS */;
INSERT INTO `maps` VALUES (31,'twolands.tga',' Two Lands 1.1\n','twolands11.map','Balanced map for a land duel.  Version 1.1.\"\n',40,2,NULL,NULL,0,934492349),(32,'Biddyn.tga',' Biddyn\n','Biddyn.map','Biddyn (v2.0)- 4th release (22/02/15) ^Suitable for 4-8 land nations.^Author: Pymous (http://www.pymous.com)^Check also Project Omniomicon, my total conversion mod project for Dominions4 at Desura or MODDB\"\n',90,9,NULL,NULL,0,3835535592);
/*!40000 ALTER TABLE `maps` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `matches`
--

DROP TABLE IF EXISTS `matches`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `matches` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `map_id` int(11) NOT NULL,
  `age` int(11) DEFAULT NULL,
  `name` varchar(255) DEFAULT NULL,
  `status` int(11) DEFAULT NULL,
  `port` int(11) DEFAULT NULL,
  `tone` int(11) DEFAULT NULL,
  `ttwo` int(11) DEFAULT NULL,
  `tthree` int(11) DEFAULT NULL,
  `points` int(11) DEFAULT NULL,
  `research_diff` int(11) DEFAULT NULL,
  `renaming` int(11) DEFAULT NULL,
  `clientstart` int(11) DEFAULT NULL,
  `password` varchar(255) DEFAULT NULL,
  `day` int(11) DEFAULT '0',
  `hour` int(11) DEFAULT '0',
  `hostinterval` int(11) DEFAULT '0',
  PRIMARY KEY (`id`),
  KEY `map_id` (`map_id`),
  CONSTRAINT `matches_ibfk_1` FOREIGN KEY (`map_id`) REFERENCES `maps` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=16 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `matches`
--

LOCK TABLES `matches` WRITE;
/*!40000 ALTER TABLE `matches` DISABLE KEYS */;
INSERT INTO `matches` VALUES (15,32,1,'matts_game',1,4096,5,0,0,5,1,1,0,'$2y$10$hF3UB3ZBkbm0PlomXgqzlOicKbAJpzQcOeUQDr6tUr9jdFXC/K1Me',0,0,0);
/*!40000 ALTER TABLE `matches` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `matchmods`
--

DROP TABLE IF EXISTS `matchmods`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `matchmods` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `match_id` int(11) NOT NULL,
  `mod_id` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `match_id` (`match_id`),
  KEY `mod_id` (`mod_id`),
  CONSTRAINT `matchmods_ibfk_1` FOREIGN KEY (`match_id`) REFERENCES `matches` (`id`),
  CONSTRAINT `matchmods_ibfk_2` FOREIGN KEY (`mod_id`) REFERENCES `mods` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `matchmods`
--

LOCK TABLES `matchmods` WRITE;
/*!40000 ALTER TABLE `matchmods` DISABLE KEYS */;
INSERT INTO `matchmods` VALUES (8,15,5);
/*!40000 ALTER TABLE `matchmods` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `matchnations`
--

DROP TABLE IF EXISTS `matchnations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `matchnations` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `nation_id` int(11) NOT NULL,
  `match_id` int(11) NOT NULL,
  `markdelete` int(11) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `nation_id` (`nation_id`),
  KEY `match_id` (`match_id`),
  CONSTRAINT `matchnations_ibfk_1` FOREIGN KEY (`nation_id`) REFERENCES `nations` (`id`),
  CONSTRAINT `matchnations_ibfk_2` FOREIGN KEY (`match_id`) REFERENCES `matches` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=14 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `matchnations`
--

LOCK TABLES `matchnations` WRITE;
/*!40000 ALTER TABLE `matchnations` DISABLE KEYS */;
/*!40000 ALTER TABLE `matchnations` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `mods`
--

DROP TABLE IF EXISTS `mods`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `mods` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) DEFAULT NULL,
  `icon` varchar(255) DEFAULT NULL,
  `version` varchar(255) DEFAULT NULL,
  `description` varchar(255) DEFAULT NULL,
  `crc32` int(10) unsigned DEFAULT NULL,
  `dmname` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `mods`
--

LOCK TABLES `mods` WRITE;
/*!40000 ALTER TABLE `mods` DISABLE KEYS */;
INSERT INTO `mods` VALUES (5,'Worthy_Heroes O5.0','./ExpandedMods/Banner/Worthy_HeroesO5.0.tga','O5.0','This mod makes many national heroes stronger and adds more heroes to the game, especially to nations that have a lack of heroes in vanilla. The list of new heroes and changes to existing heroes can be found in the mod thread at desura or dom3mods boards. ',26998680,'Worthy_Heroes_vO5.0.dm');
/*!40000 ALTER TABLE `mods` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `nations`
--

DROP TABLE IF EXISTS `nations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `nations` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) DEFAULT NULL,
  `subtitle` varchar(255) DEFAULT NULL,
  `turn_name` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=93 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `nations`
--

LOCK TABLES `nations` WRITE;
/*!40000 ALTER TABLE `nations` DISABLE KEYS */;
INSERT INTO `nations` VALUES (5,'Arcoscephale','Golden Era','early_arcoscephale'),(6,'Ermor','New Faith','early_ermor'),(7,'Ulm','New Faith','early_ulm'),(8,'Marverni','Time of Drus','early_maverni'),(9,'Sauromatia','Amazon Queens','early_sauromatia'),(10,'T\'ien Ch\'i','Spring and Autumn','early_tienchi'),(11,'Machaka','Lion Kings','early_machaka'),(12,'Mictlan','Reign of Blood','early_mictlan'),(13,'Abysia','Children of Flame','early_abysia'),(14,'Caelum','Eagle Kings','early_caelum'),(15,'C\'tis','Lizard Kings','early_ctis'),(16,'Pangaea','Age of Revelry','early_pangaea'),(17,'Agartha','Pale Ones','early_agartha'),(18,'Tir na n\'Og','Land of the Ever Young','early_tnn'),(19,'Fomoria','The Cursed Ones','early_fomoria'),(20,'Vanheim','Age of Vanir','early_vanheim'),(21,'Helheim','Dusk and Death','early_helheim'),(22,'Niefelheim','Sons of Winter','early_neifel'),(25,'Kailasa','Rise of the Ape Kings','early_kailasa'),(26,'Lanka','Land of the Demons','early_lanka'),(27,'Yomi','Oni Kings','early_yomi'),(28,'Hinnom','SOns of the Fallen','early_hinnom'),(29,'Ur','The First City','early_ur'),(30,'Berytos','The Phoenix Empire','early_berytos'),(31,'Xibalba','Vigil of the Sun','early_xibalba'),(33,'Arcoscephale','The Old Kingdom','mid_arcoscephale'),(34,'Ermor','Ashen Empire','mid_ermor'),(35,'Sceleria','The Reformed Empire','mid_sceleria'),(36,'Pythium','Emerald Empire','mid_pythium'),(37,'Man','Tower of Avalon','mid_man'),(38,'Eriu','Last of the Tuatha','mid_eriu'),(39,'Ulm','Forges of Ulm','mid_ulm'),(40,'Marignon','Fiery Justice','mid_marignon'),(41,'Mictlan','Reign of the Lawgiver','mid_mictlan'),(42,'T\'ien Ch\'i','Imperial Bureaucracy','mid_tienchi'),(43,'Machaka','Reign of Sorcerors','mid_machaka'),(44,'Agartha','Golem Cult','mid_agartha'),(45,'Abysia','Blood and Fire','mid_abysia'),(46,'Caelum','Reign of the Seraphim','mid_caelum'),(47,'C\'tis','Miasma','mid_ctis'),(48,'Pangaea','Age of Bronze','mid_pangaea'),(49,'Asphodel','Carrion Woods','mid_asphodel'),(50,'Vanheim','Arrival of Man','mid_vanheim'),(51,'Jotunheim','Iron Woods','mid_jotunheim'),(52,'Vanarus','Land of the Chuds','mid_vanarus'),(53,'Bandar Log','Land of the Apes','mid_bandar'),(54,'Shinuyama','Land of the Bakemono','mid_shinuyama'),(55,'Ashdod','Reign of the Anakim','mid_ashdod'),(57,'Nazca','Kingdom of the Sun','mid_nazca'),(58,'Xibalba','Flooded Caves','mid_xibalba'),(60,'Arcoscephale','Sibylline Guidance','late_arcoscephale'),(61,'Pythium','Serpent Cult','late_pythium'),(62,'Lemuria','Soul Gates','late_lemuria'),(63,'Man','Towers of Chelms','late_man'),(64,'Ulm','Black Forest','late_ulm'),(65,'Marignon','Conquerors of the Sea','late_marignon'),(66,'Mictlan','Blood and Rain','late_mictlan'),(67,'T\'ien Chi','Barbarian Kings','late_tienchi'),(69,'Jomon','Human Daimyos','late_jomon'),(70,'Agartha','Ktonian Dead','late_agartha'),(71,'Abysia','Blood of Humans','late_abysia'),(72,'Caelum','Return of the Raptors','late_caelum'),(73,'C\'tis','Desert Tombs','late_ctis'),(74,'Pangaea','New Era','late_pangaea'),(75,'Midgard','Age of Men','late_midgard'),(76,'Utgard','Well of Urd','late_utgard'),(77,'Bogarus','Age of Heroes','late_bogarus'),(78,'Patala','Reign of the Nagas','late_patala'),(79,'Gath','Last of the Giants','late_gath'),(80,'Ragha','Dual Kingdom','late_ragha'),(81,'Xibalba','Return of the Zotz','late_xibalba'),(83,'Atlantis','Emergence of the Deep Ones','early_atlantis'),(84,'R\'lyeh','Time of Aboleths','early_rlyeh'),(85,'Pelagia','Pearl Kings','early_pelagia'),(86,'Oceania','Coming of the Capricorns','early_oceania'),(87,'Atlantis','Kings of the Deep','mid_atlantis'),(88,'R\'lyeh','Fallen Star','mid_rlyeh'),(89,'Pelagia','Triton Kings','mid_pelagia'),(90,'Oceania','Mermidons','mid_oceania'),(91,'Atlantis','Frozen Sea','late_atlantis'),(92,'R\'lyeh','Dreamlands','late_rlyeh');
/*!40000 ALTER TABLE `nations` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `notifications`
--

DROP TABLE IF EXISTS `notifications`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `notifications` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `template` varchar(150) DEFAULT NULL,
  `vars` text,
  `user_id` int(11) DEFAULT '0',
  `state` int(11) DEFAULT '1',
  `created` datetime NOT NULL,
  `modified` datetime NOT NULL,
  `tracking_id` text,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `notifications`
--

LOCK TABLES `notifications` WRITE;
/*!40000 ALTER TABLE `notifications` DISABLE KEYS */;
/*!40000 ALTER TABLE `notifications` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `notifier_phinxlog`
--

DROP TABLE IF EXISTS `notifier_phinxlog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `notifier_phinxlog` (
  `version` bigint(20) NOT NULL,
  `start_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `end_time` timestamp NOT NULL DEFAULT '0000-00-00 00:00:00',
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `notifier_phinxlog`
--

LOCK TABLES `notifier_phinxlog` WRITE;
/*!40000 ALTER TABLE `notifier_phinxlog` DISABLE KEYS */;
INSERT INTO `notifier_phinxlog` VALUES (20150612200406,'2015-12-06 21:35:28','2015-12-06 21:35:28'),(20150716091509,'2015-12-06 21:35:28','2015-12-06 21:35:28');
/*!40000 ALTER TABLE `notifier_phinxlog` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `settings_configurations`
--

DROP TABLE IF EXISTS `settings_configurations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `settings_configurations` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(100) NOT NULL DEFAULT '',
  `value` text,
  `description` text,
  `type` varchar(50) NOT NULL DEFAULT '',
  `editable` int(11) NOT NULL DEFAULT '1',
  `weight` int(11) NOT NULL DEFAULT '0',
  `autoload` int(11) NOT NULL DEFAULT '1',
  `created` datetime NOT NULL,
  `modified` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `settings_configurations`
--

LOCK TABLES `settings_configurations` WRITE;
/*!40000 ALTER TABLE `settings_configurations` DISABLE KEYS */;
INSERT INTO `settings_configurations` VALUES (1,'App.Name','CakeAdmin Application',NULL,'',1,0,1,'2015-12-07 05:35:46','2015-12-07 05:35:46'),(2,'LS.subtheme','Cosmo',NULL,'select',1,0,1,'2015-12-07 05:35:46','2015-12-07 05:35:46'),(3,'LS.navbar','navbar-default',NULL,'select',1,0,1,'2015-12-07 05:35:46','2015-12-07 05:35:46'),(4,'LS.container','container',NULL,'select',1,0,1,'2015-12-07 05:35:46','2015-12-07 05:35:46');
/*!40000 ALTER TABLE `settings_configurations` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `settings_phinxlog`
--

DROP TABLE IF EXISTS `settings_phinxlog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `settings_phinxlog` (
  `version` bigint(20) NOT NULL,
  `start_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `end_time` timestamp NOT NULL DEFAULT '0000-00-00 00:00:00',
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `settings_phinxlog`
--

LOCK TABLES `settings_phinxlog` WRITE;
/*!40000 ALTER TABLE `settings_phinxlog` DISABLE KEYS */;
INSERT INTO `settings_phinxlog` VALUES (20150126111319,'2015-12-06 21:35:28','2015-12-06 21:35:28');
/*!40000 ALTER TABLE `settings_phinxlog` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `email` varchar(50) DEFAULT NULL,
  `password` varchar(255) DEFAULT NULL,
  `cakeadmin` int(11) DEFAULT '0',
  `request_key` varchar(255) DEFAULT NULL,
  `created` datetime NOT NULL,
  `modified` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

LOCK TABLES `users` WRITE;
/*!40000 ALTER TABLE `users` DISABLE KEYS */;
INSERT INTO `users` VALUES (1,'mattkramara@gmail.com','$2y$10$/CXoAkT2em8dg/gbJaitAu/tfDHwEZH37/EjIcI33eIKCwSDVkckS',1,NULL,'2015-12-07 05:35:52','2015-12-07 05:35:52');
/*!40000 ALTER TABLE `users` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2016-02-20  1:56:53
