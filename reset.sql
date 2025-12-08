SET FOREIGN_KEY_CHECKS = 0;

SET @tables = (
    SELECT GROUP_CONCAT(table_name)
    FROM information_schema.tables
    WHERE table_schema = DATABASE()
);

SET @stmt = CONCAT('DROP TABLE IF EXISTS ', @tables);
PREPARE dropstmt FROM @stmt;
EXECUTE dropstmt;
DEALLOCATE PREPARE dropstmt;

SET FOREIGN_KEY_CHECKS = 1;
