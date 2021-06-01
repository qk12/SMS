--院系表
create table department
(
    yxh char(2) not null,
    mc varchar(100),
    dz varchar(100),
    lxdh varchar(100),
    primary key (yxh)
);

INSERT INTO 
department
VALUES
('01','计算机学院','上大东校区三号楼','65347567'),
('02','通讯学院','上大东校区二号楼','65341234'),
('03','材料学院','上大东校区四号楼','65347890');


--管理员
create table admin
(
    account varchar(100) not null,
    password varchar(100),
    primary key (account)
);

INSERT INTO 
admin
VALUES
('admin', 'admin');


--教师表
create table teacher
(
    gh char(4) not null,
    xm varchar(100),
    xb char(4),
    csrq date,
    zc varchar(100) ,
    yxh char(2) not null,
    primary key (gh),
    foreign key (yxh) references Department(yxh)
);

INSERT INTO 
teacher
VALUES
('0101','陈迪茂','男','1973-03-06','副教授','01'),
('0102','马小红','女','1972-12-08','讲师','01'),
('0201','张心颖','女','1960-01-05','教授','02'),
('0103','吴宝钢','男','1980-11-06','讲师','01');


--课程表
create table class
(
    kh char(8),
    km varchar(100),
    xf int,
    xs int,
    yxh nchar(2),
    primary key(kh),
    foreign key (yxh) references department(yxh)
);

Insert into
class
values
('08305001','离散数学',4,40,'01'),
('08305002','数据库原理',4,50,'01'),
('08305003','数据结构',4,50,'01'),
('08305004','系统结构',6,60,'01'),
('08301001','分子物理学',4,40,'03'),
('08302001','通信学',3,30,'02');


--开课表O
create table openclass
(
    xq varchar(100) not null,
    kh char(8) not null,
    gh char(4),
    sksj varchar(100),
    primary key (xq, kh, gh),
    foreign key (kh) references class(kh),
    foreign key (gh) references teacher(gh)
);

Insert into
openclass
values
('2012-2013 冬季', '08305002', '0101', '星期三 1-4'),
('2012-2013 冬季', '08305002', '0102', '星期三 1-4'),
('2012-2013 冬季', '08305002', '0103', '星期三 1-4'),
('2012-2013 冬季', '08305003', '0102', '星期五 5-6'),
('2012-2013 秋季', '08305001', '0103', '星期三 5-6'),
('2013-2014 冬季', '08301001', '0101', '星期五 1-2'),
('2013-2014 冬季', '08302001', '0201', '星期一 5-8'),
('2013-2014 冬季', '08305001', '0101', '星期一 1-2'),
('2013-2014 秋季', '08305001', '0102', '星期一 5-6'),
('2013-2014 秋季', '08305004', '0101', '星期二 1-4');


--学生表
create table student
(
    xh char(4) not null,
    xm varchar(100),
    xb char(4),
    csrq date,
    jg varchar(100),
    sjhm varchar(100),
    yxh char(2) not null,
    primary key (xh),
    foreign key (yxh) references department(yxh)
);

INSERT INTO 
student
VALUES
('1101','李明','男','1993-03-06','上海','13613005486','02'),
('1102','刘晓明','男','1992-12-08','安徽','18913457890','01'),
('1103','张颖','女','1993-01-05','江苏','18826490423','01'),
('1104','刘晶晶','女','1994-11-06','上海','13331934111','01'),
('1105','刘成刚','男','1991-06-07','上海','18015872567','01'),
('1106','李二丽','女','1993-05-04','江苏','18107620945','01'),
('1107','张晓峰','男','1992-08-16','浙江','13912341078','01');


-- 学期表 terms
CREATE TABLE terms
(
    term varchar(100) NOT NULL,
    id serial NOT NULL,
    primary key (id)
);

-- Records of terms
INSERT INTO
terms
VALUES
('2012-2013 冬季', 2),
('2012-2013 秋季', 1),
('2013-2014 冬季', 4),
('2013-2014 秋季', 3);


--选课表E
create table xuankeTable
(
    xh char(4) not null,
    xq varchar(100) not null,
    kh char(8) not null,
    gh char(4) not null,
    zpcj int check(1<=zpcj and zpcj<=100),
    primary key (xh, xq, kh, gh)
);

Insert into
xuankeTable
values
('1101', '2012-2013 秋季', '08305001', '0103', 60),
('1101', '2013-2014 冬季', '08301001', '0101', 87),
('1101', '2013-2014 冬季', '08302001', '0201', 82),
('1102', '2012-2013 秋季', '08305001', '0103', 56),
('1102', '2013-2014 秋季', '08305004', '0101', 75),
('1103', '2012-2013 冬季', '08305002', '0102', 84),
('1103', '2012-2013 冬季', '08305003', '0102', null),
('1103', '2012-2013 秋季', '08305001', '0103', null),
('1103', '2013-2014 秋季', '08305004', '0101', 74),
('1104', '2012-2013 秋季', '08305001', '0103', null),
('1104', '2013-2014 冬季', '08302001', '0201', 85),
('1106', '2012-2013 冬季', '08305002', '0103', 66),
('1106', '2012-2013 秋季', '08305001', '0103', 90),
('1107', '2013-2014 秋季', '08305004', '0101', 79);


/*
-- ----------------------------
-- Procedure structure for change_zpcj
-- ----------------------------
create procedure change_zpcj(in xhs varchar(256))
begin
    update xuankeTable set zpcj=(pscj *0.7 + kscj * 0.3) where xh=xhs;
end

-- ----------------------------
-- Triggers structure for table xuankeTable
-- ----------------------------
create trigger tri_zpcj after update on xuankeTable for each row
begin
  declare g_avg int;
  set g_avg = (select avg(zpcj) from xuankeTable where xh=new.xh);
  update student set grade=g_avg where xh=new.xh;
end
*/